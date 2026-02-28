use std::collections::HashMap;
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");

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

const REQUIRED_ACTIONS: &[&str] = &[
    "new_split:right",
    "new_split:down",
    "goto_split:next",
    "goto_split:previous",
    "equalize_splits",
];

#[derive(Debug, PartialEq)]
struct Keybindings {
    split_right: String,
    split_down: String,
    goto_next: String,
    goto_previous: String,
    equalize: String,
}

fn find_config_path() -> Result<PathBuf, String> {
    let home =
        std::env::var("HOME").map_err(|_| "HOME environment variable is not set".to_string())?;

    let mut candidates = Vec::new();

    #[cfg(target_os = "macos")]
    {
        candidates.push(
            PathBuf::from(&home).join("Library/Application Support/com.mitchellh.ghostty/config"),
        );
    }

    let xdg_config =
        std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| format!("{}/.config", home));
    candidates.push(PathBuf::from(&xdg_config).join("ghostty/config"));

    for path in &candidates {
        if path.exists() {
            return Ok(path.clone());
        }
    }

    Err(format!(
        "Ghostty config file not found. Searched:\n{}",
        candidates
            .iter()
            .map(|p| format!("  - {}", p.display()))
            .collect::<Vec<_>>()
            .join("\n")
    ))
}

fn parse_keybindings(content: &str) -> Result<Keybindings, String> {
    let mut bindings: HashMap<&str, &str> = HashMap::new();

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

        // trigger=action の分割
        let Some((trigger, action)) = rest.split_once('=') else {
            continue;
        };
        let trigger = trigger.trim();
        let action = action.trim();

        if REQUIRED_ACTIONS.contains(&action) {
            bindings.insert(action, trigger);
        }
    }

    let missing: Vec<&str> = REQUIRED_ACTIONS
        .iter()
        .filter(|a| !bindings.contains_key(*a))
        .copied()
        .collect();

    if !missing.is_empty() {
        return Err(format!(
            "Missing keybindings for the following actions:\n{}\n\n\
             Add them to your Ghostty config. Example:\n\
             \x20 keybind = super+d=new_split:right\n\
             \x20 keybind = super+shift+d=new_split:down\n\
             \x20 keybind = super+ctrl+right_bracket=goto_split:next\n\
             \x20 keybind = super+ctrl+left_bracket=goto_split:previous\n\
             \x20 keybind = super+ctrl+shift+equal=equalize_splits",
            missing
                .iter()
                .map(|a| format!("  - {}", a))
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }

    Ok(Keybindings {
        split_right: bindings["new_split:right"].to_string(),
        split_down: bindings["new_split:down"].to_string(),
        goto_next: bindings["goto_split:next"].to_string(),
        goto_previous: bindings["goto_split:previous"].to_string(),
        equalize: bindings["equalize_splits"].to_string(),
    })
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
                let config_path = match find_config_path() {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                };
                let content = match std::fs::read_to_string(&config_path) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Error: Failed to read {}: {}", config_path.display(), e);
                        std::process::exit(1);
                    }
                };
                let keybindings = match parse_keybindings(&content) {
                    Ok(k) => k,
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
                println!("Keybindings: {:?}", keybindings);
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

    const FULL_CONFIG: &str = "\
keybind = super+d=new_split:right
keybind = super+shift+d=new_split:down
keybind = super+ctrl+right_bracket=goto_split:next
keybind = super+ctrl+left_bracket=goto_split:previous
keybind = super+ctrl+shift+equal=equalize_splits
";

    #[test]
    fn parse_keybindings_all_present() {
        let kb = parse_keybindings(FULL_CONFIG).unwrap();
        assert_eq!(kb.split_right, "super+d");
        assert_eq!(kb.split_down, "super+shift+d");
        assert_eq!(kb.goto_next, "super+ctrl+right_bracket");
        assert_eq!(kb.goto_previous, "super+ctrl+left_bracket");
        assert_eq!(kb.equalize, "super+ctrl+shift+equal");
    }

    #[test]
    fn parse_keybindings_with_comments_and_other_lines() {
        let config = "\
# This is a comment
font-size = 14
keybind = super+d=new_split:right

keybind = super+shift+d=new_split:down
# another comment
keybind = super+ctrl+right_bracket=goto_split:next
keybind = super+ctrl+left_bracket=goto_split:previous
keybind = super+ctrl+shift+equal=equalize_splits
keybind = super+t=new_tab
";
        let kb = parse_keybindings(config).unwrap();
        assert_eq!(kb.split_right, "super+d");
        assert_eq!(kb.split_down, "super+shift+d");
    }

    #[test]
    fn parse_keybindings_with_extra_whitespace() {
        let config = "\
  keybind = super+d = new_split:right
keybind =   super+shift+d = new_split:down
keybind = super+ctrl+right_bracket = goto_split:next
keybind = super+ctrl+left_bracket = goto_split:previous
keybind = super+ctrl+shift+equal = equalize_splits
";
        let kb = parse_keybindings(config).unwrap();
        assert_eq!(kb.split_right, "super+d");
        assert_eq!(kb.split_down, "super+shift+d");
    }

    #[test]
    fn parse_keybindings_missing_some() {
        let config = "\
keybind = super+d=new_split:right
keybind = super+shift+d=new_split:down
";
        let err = parse_keybindings(config).unwrap_err();
        assert!(err.contains("goto_split:next"), "error: {}", err);
        assert!(err.contains("goto_split:previous"), "error: {}", err);
        assert!(err.contains("equalize_splits"), "error: {}", err);
    }

    #[test]
    fn parse_keybindings_empty_config() {
        let err = parse_keybindings("").unwrap_err();
        for action in REQUIRED_ACTIONS {
            assert!(err.contains(action), "error should contain {}", action);
        }
    }

    #[test]
    fn parse_keybindings_last_binding_wins() {
        let config = "\
keybind = super+d=new_split:right
keybind = ctrl+d=new_split:right
keybind = super+shift+d=new_split:down
keybind = super+ctrl+right_bracket=goto_split:next
keybind = super+ctrl+left_bracket=goto_split:previous
keybind = super+ctrl+shift+equal=equalize_splits
";
        let kb = parse_keybindings(config).unwrap();
        assert_eq!(kb.split_right, "ctrl+d");
    }
}
