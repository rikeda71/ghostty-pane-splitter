use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const REQUIRED_ACTIONS: &[&str] = &[
    "new_split:right",
    "new_split:down",
    "goto_split:next",
    "goto_split:previous",
    "equalize_splits",
];

#[derive(Debug, PartialEq)]
struct Layout {
    cols: u32,
    rows: u32,
}

fn parse_layout(arg: &str) -> Result<Layout, String> {
    // グリッド指定: CxR 形式
    if let Some((cols_str, rows_str)) = arg.split_once('x') {
        let cols = cols_str
            .parse::<u32>()
            .map_err(|_| format!("Invalid grid format: '{}'", arg))?;
        let rows = rows_str
            .parse::<u32>()
            .map_err(|_| format!("Invalid grid format: '{}'", arg))?;
        if cols == 0 || rows == 0 {
            return Err(format!("Grid dimensions must be >= 1, got: '{}'", arg));
        }
        if cols * rows < 2 {
            return Err("Total panes must be >= 2".to_string());
        }
        return Ok(Layout { cols, rows });
    }

    // 数値指定
    let n = arg.parse::<u32>().map_err(|_| {
        format!(
            "Invalid argument: '{}'. Expected a number or grid spec (e.g. 4, 2x3)",
            arg
        )
    })?;
    if n < 2 {
        return Err("Number of panes must be >= 2".to_string());
    }

    // 正方形に近い因数分解: √N に最も近い因数ペアを選択
    let sqrt = (n as f64).sqrt().ceil() as u32;
    let mut cols = sqrt;
    while n % cols != 0 {
        cols += 1;
    }
    let rows = n / cols;
    // cols >= rows になるよう調整
    let (cols, rows) = if cols >= rows {
        (cols, rows)
    } else {
        (rows, cols)
    };

    Ok(Layout { cols, rows })
}

fn find_config_path() -> Option<PathBuf> {
    if cfg!(target_os = "macos") {
        if let Some(home) = std::env::var_os("HOME") {
            let path = PathBuf::from(home)
                .join("Library/Application Support/com.mitchellh.ghostty/config");
            if path.exists() {
                return Some(path);
            }
        }
    }

    let xdg_config = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_default();
            PathBuf::from(home).join(".config")
        });
    let path = xdg_config.join("ghostty/config");
    if path.exists() {
        return Some(path);
    }

    None
}

fn parse_keybinds(content: &str) -> HashMap<String, String> {
    let mut keybinds = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some(rest) = line.strip_prefix("keybind") else {
            continue;
        };
        let rest = rest.trim();
        let Some(rest) = rest.strip_prefix('=') else {
            continue;
        };
        let rest = rest.trim();
        if let Some((trigger, action)) = rest.split_once('=') {
            keybinds.insert(action.trim().to_string(), trigger.trim().to_string());
        }
    }
    keybinds
}

fn validate_keybinds(keybinds: &HashMap<String, String>) -> Result<(), Vec<&'static str>> {
    let missing: Vec<&str> = REQUIRED_ACTIONS
        .iter()
        .filter(|action| !keybinds.contains_key(**action))
        .copied()
        .collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(missing)
    }
}

fn load_keybinds() -> Result<HashMap<String, String>, String> {
    let config_path = find_config_path().ok_or_else(|| {
        "Ghostty config file not found. Expected locations:\n  \
         macOS: ~/Library/Application Support/com.mitchellh.ghostty/config\n  \
         Linux: ~/.config/ghostty/config"
            .to_string()
    })?;

    let content = fs::read_to_string(&config_path).map_err(|e| {
        format!(
            "Failed to read config file '{}': {}",
            config_path.display(),
            e
        )
    })?;

    let keybinds = parse_keybinds(&content);

    if let Err(missing) = validate_keybinds(&keybinds) {
        let mut msg = format!("Missing keybinds in {}:\n", config_path.display());
        for action in &missing {
            msg.push_str(&format!("  - {}\n", action));
        }
        msg.push_str("\nAdd the following to your Ghostty config:\n");
        msg.push_str("  keybind = super+d=new_split:right\n");
        msg.push_str("  keybind = super+shift+d=new_split:down\n");
        msg.push_str("  keybind = super+ctrl+right_bracket=goto_split:next\n");
        msg.push_str("  keybind = super+ctrl+left_bracket=goto_split:previous\n");
        msg.push_str("  keybind = super+ctrl+shift+equal=equalize_splits");
        return Err(msg);
    }

    Ok(keybinds)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    match args[1].as_str() {
        "--help" | "-h" => print_usage(),
        "--version" | "-V" => println!("ghostty-pane-splitter {}", VERSION),
        arg => match parse_layout(arg) {
            Ok(layout) => {
                let _keybinds = match load_keybinds() {
                    Ok(kb) => kb,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                };
                println!(
                    "Grid: {}x{} ({} panes)",
                    layout.cols,
                    layout.rows,
                    layout.cols * layout.rows
                );
                println!("(Not yet implemented)");
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
    }
}

fn print_usage() {
    println!(
        "ghostty-pane-splitter {}
CLI tool to split panes on Ghostty Terminal

USAGE:
    ghostty-pane-splitter <LAYOUT>

ARGS:
    <LAYOUT>    Number of panes (e.g. 4) or grid spec (e.g. 2x3)

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

EXAMPLES:
    ghostty-pane-splitter 4      # Split into 2x2 grid
    ghostty-pane-splitter 2x3    # Split into 2 cols x 3 rows",
        VERSION
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_layout_valid_cases() {
        let cases = [
            // 数値指定
            ("2", 2, 1),
            ("3", 3, 1),
            ("4", 2, 2),
            ("5", 5, 1), // 素数は横一列
            ("6", 3, 2),
            ("9", 3, 3),
            // グリッド指定
            ("2x3", 2, 3),
            ("3x2", 3, 2),
            ("1x4", 1, 4),
        ];
        for (input, cols, rows) in cases {
            assert_eq!(
                parse_layout(input),
                Ok(Layout { cols, rows }),
                "input: {}",
                input
            );
        }
    }

    #[test]
    fn parse_layout_invalid_cases() {
        let cases = ["abc", "0", "1", "0x3", "2x0", "1x1", "axb"];
        for input in cases {
            assert!(parse_layout(input).is_err(), "input: {}", input);
        }
    }

    #[test]
    fn parse_keybinds_valid() {
        let content = "\
keybind = super+d=new_split:right
keybind = super+shift+d=new_split:down
keybind = super+ctrl+right_bracket=goto_split:next
keybind = super+ctrl+left_bracket=goto_split:previous
keybind = super+ctrl+shift+equal=equalize_splits
";
        let keybinds = parse_keybinds(content);
        assert_eq!(keybinds.get("new_split:right").unwrap(), "super+d");
        assert_eq!(keybinds.get("new_split:down").unwrap(), "super+shift+d");
        assert_eq!(
            keybinds.get("goto_split:next").unwrap(),
            "super+ctrl+right_bracket"
        );
        assert_eq!(
            keybinds.get("goto_split:previous").unwrap(),
            "super+ctrl+left_bracket"
        );
        assert_eq!(
            keybinds.get("equalize_splits").unwrap(),
            "super+ctrl+shift+equal"
        );
    }

    #[test]
    fn parse_keybinds_with_comments_and_other_settings() {
        let content = "\
# This is a comment
font-size = 14
keybind = super+d=new_split:right

# Another comment
window-decoration = false
keybind = super+shift+d=new_split:down
";
        let keybinds = parse_keybinds(content);
        assert_eq!(keybinds.len(), 2);
        assert_eq!(keybinds.get("new_split:right").unwrap(), "super+d");
        assert_eq!(keybinds.get("new_split:down").unwrap(), "super+shift+d");
    }

    #[test]
    fn parse_keybinds_duplicate_action_last_wins() {
        let content = "\
keybind = super+d=new_split:right
keybind = ctrl+d=new_split:right
";
        let keybinds = parse_keybinds(content);
        assert_eq!(keybinds.get("new_split:right").unwrap(), "ctrl+d");
    }

    #[test]
    fn parse_keybinds_empty_content() {
        let keybinds = parse_keybinds("");
        assert!(keybinds.is_empty());
    }

    #[test]
    fn parse_keybinds_no_spaces() {
        let content = "keybind=super+d=new_split:right\n";
        let keybinds = parse_keybinds(content);
        assert_eq!(keybinds.get("new_split:right").unwrap(), "super+d");
    }

    #[test]
    fn validate_keybinds_all_present() {
        let mut keybinds = HashMap::new();
        keybinds.insert("new_split:right".to_string(), "super+d".to_string());
        keybinds.insert("new_split:down".to_string(), "super+shift+d".to_string());
        keybinds.insert("goto_split:next".to_string(), "super+ctrl+]".to_string());
        keybinds.insert(
            "goto_split:previous".to_string(),
            "super+ctrl+[".to_string(),
        );
        keybinds.insert(
            "equalize_splits".to_string(),
            "super+ctrl+shift+=".to_string(),
        );
        assert!(validate_keybinds(&keybinds).is_ok());
    }

    #[test]
    fn validate_keybinds_missing_all() {
        let keybinds = HashMap::new();
        let result = validate_keybinds(&keybinds);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().len(), 5);
    }

    #[test]
    fn validate_keybinds_partial() {
        let mut keybinds = HashMap::new();
        keybinds.insert("new_split:right".to_string(), "super+d".to_string());
        keybinds.insert("new_split:down".to_string(), "super+shift+d".to_string());
        let result = validate_keybinds(&keybinds);
        assert!(result.is_err());
        let missing = result.unwrap_err();
        assert_eq!(missing.len(), 3);
        assert!(missing.contains(&"goto_split:next"));
        assert!(missing.contains(&"goto_split:previous"));
        assert!(missing.contains(&"equalize_splits"));
    }
}
