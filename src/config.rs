use std::collections::HashMap;
use std::path::PathBuf;

use crate::keybind::{parse_key_combo, KeyCombo};

const REQUIRED_ACTIONS: &[&str] = &[
    "new_split:right",
    "new_split:down",
    "goto_split:next",
    "goto_split:previous",
    "equalize_splits",
];

/// Parsed keybindings required for pane splitting operations.
#[derive(Debug, PartialEq)]
pub struct Keybindings {
    /// Key combo for splitting a pane to the right.
    pub split_right: KeyCombo,
    /// Key combo for splitting a pane downward.
    pub split_down: KeyCombo,
    /// Key combo for navigating to the next pane.
    pub goto_next: KeyCombo,
    /// Key combo for navigating to the previous pane.
    pub goto_previous: KeyCombo,
    /// Key combo for equalizing pane sizes.
    pub equalize: KeyCombo,
}

/// Finds the Ghostty config file path by checking platform-specific locations.
pub fn find_config_path() -> Result<PathBuf, String> {
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

/// Parses Ghostty config content and extracts the required keybindings.
pub fn parse_keybindings(content: &str) -> Result<Keybindings, String> {
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

#[cfg(test)]
mod tests {
    use enigo::Key;

    use super::*;

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
}
