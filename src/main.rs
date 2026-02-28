use enigo::Key;
use std::collections::HashMap;
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

#[derive(Debug, PartialEq)]
struct KeyCombo {
    modifiers: Vec<Key>,
    key: Key,
}

fn parse_ghostty_key(name: &str) -> Result<Key, String> {
    match name {
        // 修飾キー
        "super" => Ok(Key::Meta),
        "ctrl" | "control" => Ok(Key::Control),
        "shift" => Ok(Key::Shift),
        "alt" => Ok(Key::Alt),

        // 特殊キー
        "space" => Ok(Key::Space),
        "tab" => Ok(Key::Tab),
        "return" | "enter" => Ok(Key::Return),
        "escape" | "esc" => Ok(Key::Escape),
        "backspace" => Ok(Key::Backspace),
        "delete" => Ok(Key::Delete),
        "home" => Ok(Key::Home),
        "end" => Ok(Key::End),
        "page_up" => Ok(Key::PageUp),
        "page_down" => Ok(Key::PageDown),

        // 矢印キー
        "up" => Ok(Key::UpArrow),
        "down" => Ok(Key::DownArrow),
        "left" => Ok(Key::LeftArrow),
        "right" => Ok(Key::RightArrow),

        // ファンクションキー
        "f1" => Ok(Key::F1),
        "f2" => Ok(Key::F2),
        "f3" => Ok(Key::F3),
        "f4" => Ok(Key::F4),
        "f5" => Ok(Key::F5),
        "f6" => Ok(Key::F6),
        "f7" => Ok(Key::F7),
        "f8" => Ok(Key::F8),
        "f9" => Ok(Key::F9),
        "f10" => Ok(Key::F10),
        "f11" => Ok(Key::F11),
        "f12" => Ok(Key::F12),

        // 記号キー (Ghostty の命名規則)
        "left_bracket" => Ok(Key::Unicode('[')),
        "right_bracket" => Ok(Key::Unicode(']')),
        "equal" => Ok(Key::Unicode('=')),
        "minus" => Ok(Key::Unicode('-')),
        "comma" => Ok(Key::Unicode(',')),
        "period" => Ok(Key::Unicode('.')),
        "slash" => Ok(Key::Unicode('/')),
        "backslash" => Ok(Key::Unicode('\\')),
        "semicolon" => Ok(Key::Unicode(';')),
        "apostrophe" => Ok(Key::Unicode('\'')),
        "grave_accent" => Ok(Key::Unicode('`')),

        // 1文字の場合は Unicode として扱う
        s if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            Ok(Key::Unicode(c))
        }

        _ => Err(format!("Unknown key: '{}'", name)),
    }
}

const MODIFIER_NAMES: &[&str] = &["super", "ctrl", "control", "shift", "alt"];

fn parse_key_combo(trigger: &str) -> Result<KeyCombo, String> {
    if trigger.is_empty() {
        return Err("Empty keybinding".to_string());
    }
    let parts: Vec<&str> = trigger.split('+').collect();

    // 最後の要素がメインキー、それ以外は修飾キー
    let mut modifiers = Vec::new();
    for &part in &parts[..parts.len() - 1] {
        if !MODIFIER_NAMES.contains(&part) {
            return Err(format!(
                "Expected modifier key, got '{}'. Valid modifiers: super, ctrl, shift, alt",
                part
            ));
        }
        modifiers.push(parse_ghostty_key(part)?);
    }

    let key = parse_ghostty_key(parts[parts.len() - 1])?;

    Ok(KeyCombo { modifiers, key })
}

#[derive(Debug, PartialEq)]
struct Keybindings {
    split_right: KeyCombo,
    split_down: KeyCombo,
    goto_next: KeyCombo,
    goto_previous: KeyCombo,
    equalize: KeyCombo,
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
        split_right: parse_key_combo(bindings["new_split:right"])?,
        split_down: parse_key_combo(bindings["new_split:down"])?,
        goto_next: parse_key_combo(bindings["goto_split:next"])?,
        goto_previous: parse_key_combo(bindings["goto_split:previous"])?,
        equalize: parse_key_combo(bindings["equalize_splits"])?,
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
        assert_eq!(
            kb.split_right,
            KeyCombo {
                modifiers: vec![Key::Meta],
                key: Key::Unicode('d')
            }
        );
        assert_eq!(
            kb.split_down,
            KeyCombo {
                modifiers: vec![Key::Meta, Key::Shift],
                key: Key::Unicode('d')
            }
        );
        assert_eq!(
            kb.goto_next,
            KeyCombo {
                modifiers: vec![Key::Meta, Key::Control],
                key: Key::Unicode(']')
            }
        );
        assert_eq!(
            kb.goto_previous,
            KeyCombo {
                modifiers: vec![Key::Meta, Key::Control],
                key: Key::Unicode('[')
            }
        );
        assert_eq!(
            kb.equalize,
            KeyCombo {
                modifiers: vec![Key::Meta, Key::Control, Key::Shift],
                key: Key::Unicode('=')
            }
        );
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
        assert_eq!(
            kb.split_right,
            KeyCombo {
                modifiers: vec![Key::Meta],
                key: Key::Unicode('d')
            }
        );
        assert_eq!(
            kb.split_down,
            KeyCombo {
                modifiers: vec![Key::Meta, Key::Shift],
                key: Key::Unicode('d')
            }
        );
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
        assert_eq!(
            kb.split_right,
            KeyCombo {
                modifiers: vec![Key::Meta],
                key: Key::Unicode('d')
            }
        );
        assert_eq!(
            kb.split_down,
            KeyCombo {
                modifiers: vec![Key::Meta, Key::Shift],
                key: Key::Unicode('d')
            }
        );
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
        assert_eq!(
            kb.split_right,
            KeyCombo {
                modifiers: vec![Key::Control],
                key: Key::Unicode('d')
            }
        );
    }

    #[test]
    fn parse_key_combo_valid_cases() {
        let cases = [
            // 修飾キー1つ + 文字キー
            (
                "super+d",
                KeyCombo {
                    modifiers: vec![Key::Meta],
                    key: Key::Unicode('d'),
                },
            ),
            (
                "ctrl+c",
                KeyCombo {
                    modifiers: vec![Key::Control],
                    key: Key::Unicode('c'),
                },
            ),
            (
                "shift+a",
                KeyCombo {
                    modifiers: vec![Key::Shift],
                    key: Key::Unicode('a'),
                },
            ),
            (
                "alt+x",
                KeyCombo {
                    modifiers: vec![Key::Alt],
                    key: Key::Unicode('x'),
                },
            ),
            // 修飾キー複数
            (
                "super+shift+d",
                KeyCombo {
                    modifiers: vec![Key::Meta, Key::Shift],
                    key: Key::Unicode('d'),
                },
            ),
            (
                "super+ctrl+shift+equal",
                KeyCombo {
                    modifiers: vec![Key::Meta, Key::Control, Key::Shift],
                    key: Key::Unicode('='),
                },
            ),
            // 修飾キーなし
            (
                "space",
                KeyCombo {
                    modifiers: vec![],
                    key: Key::Space,
                },
            ),
            (
                "f1",
                KeyCombo {
                    modifiers: vec![],
                    key: Key::F1,
                },
            ),
            // 記号キー
            (
                "super+ctrl+right_bracket",
                KeyCombo {
                    modifiers: vec![Key::Meta, Key::Control],
                    key: Key::Unicode(']'),
                },
            ),
            (
                "super+ctrl+left_bracket",
                KeyCombo {
                    modifiers: vec![Key::Meta, Key::Control],
                    key: Key::Unicode('['),
                },
            ),
            // 矢印キー
            (
                "ctrl+up",
                KeyCombo {
                    modifiers: vec![Key::Control],
                    key: Key::UpArrow,
                },
            ),
            // control エイリアス
            (
                "control+d",
                KeyCombo {
                    modifiers: vec![Key::Control],
                    key: Key::Unicode('d'),
                },
            ),
            // enter エイリアス
            (
                "super+enter",
                KeyCombo {
                    modifiers: vec![Key::Meta],
                    key: Key::Return,
                },
            ),
        ];
        for (input, expected) in cases {
            assert_eq!(
                parse_key_combo(input).unwrap(),
                expected,
                "input: {}",
                input
            );
        }
    }

    #[test]
    fn parse_key_combo_invalid_cases() {
        let cases = [
            ("", "Empty keybinding"),
            ("super+unknown_key", "Unknown key"),
            ("invalid_mod+d", "Expected modifier key"),
        ];
        for (input, expected_substring) in cases {
            let err = parse_key_combo(input).unwrap_err();
            assert!(
                err.contains(expected_substring),
                "input: '{}', error: '{}', expected to contain: '{}'",
                input,
                err,
                expected_substring
            );
        }
    }

    #[test]
    fn parse_ghostty_key_valid_cases() {
        let cases = [
            // 修飾キー
            ("super", Key::Meta),
            ("ctrl", Key::Control),
            ("control", Key::Control),
            ("shift", Key::Shift),
            ("alt", Key::Alt),
            // 特殊キー
            ("space", Key::Space),
            ("tab", Key::Tab),
            ("return", Key::Return),
            ("enter", Key::Return),
            ("escape", Key::Escape),
            ("esc", Key::Escape),
            ("backspace", Key::Backspace),
            ("delete", Key::Delete),
            ("home", Key::Home),
            ("end", Key::End),
            ("page_up", Key::PageUp),
            ("page_down", Key::PageDown),
            // 矢印キー
            ("up", Key::UpArrow),
            ("down", Key::DownArrow),
            ("left", Key::LeftArrow),
            ("right", Key::RightArrow),
            // ファンクションキー
            ("f1", Key::F1),
            ("f12", Key::F12),
            // 記号キー
            ("left_bracket", Key::Unicode('[')),
            ("right_bracket", Key::Unicode(']')),
            ("equal", Key::Unicode('=')),
            ("minus", Key::Unicode('-')),
            ("comma", Key::Unicode(',')),
            ("period", Key::Unicode('.')),
            ("slash", Key::Unicode('/')),
            ("backslash", Key::Unicode('\\')),
            ("semicolon", Key::Unicode(';')),
            ("apostrophe", Key::Unicode('\'')),
            ("grave_accent", Key::Unicode('`')),
            // 1文字
            ("d", Key::Unicode('d')),
            ("1", Key::Unicode('1')),
        ];
        for (input, expected) in cases {
            assert_eq!(
                parse_ghostty_key(input).unwrap(),
                expected,
                "input: {}",
                input
            );
        }
    }

    #[test]
    fn parse_ghostty_key_invalid_cases() {
        let cases = ["unknown_key", "foobar", ""];
        for input in cases {
            assert!(
                parse_ghostty_key(input).is_err(),
                "input: '{}' should fail",
                input
            );
        }
    }
}
