use enigo::Key;

/// A key combination consisting of modifier keys and a primary key.
#[derive(Debug, PartialEq)]
pub struct KeyCombo {
    /// Modifier keys (e.g. Ctrl, Shift, Meta).
    pub modifiers: Vec<Key>,
    /// The primary key to press.
    pub key: Key,
}

/// Returns the appropriate Key for a Ghostty named symbol key.
/// On macOS, uses the physical keycode (kVK_ANSI_*) to match Ghostty's keybinding system.
/// On other platforms, uses the Unicode character with layout-dependent resolution.
pub(crate) fn physical_key(macos_keycode: u32, unicode_char: char) -> Key {
    if cfg!(target_os = "macos") {
        Key::Other(macos_keycode)
    } else {
        Key::Unicode(unicode_char)
    }
}

/// Converts a Ghostty key name string into an enigo `Key`.
fn parse_ghostty_key(name: &str) -> Result<Key, String> {
    match name {
        // Modifier keys
        "super" => Ok(Key::Meta),
        "ctrl" | "control" => Ok(Key::Control),
        "shift" => Ok(Key::Shift),
        "alt" => Ok(Key::Alt),

        // Special keys
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

        // Arrow keys
        "up" => Ok(Key::UpArrow),
        "down" => Ok(Key::DownArrow),
        "left" => Ok(Key::LeftArrow),
        "right" => Ok(Key::RightArrow),

        // Function keys
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

        // Symbol keys (Ghostty naming) - uses physical keycodes (kVK_ANSI_*) on macOS
        "left_bracket" => Ok(physical_key(0x21, '[')),
        "right_bracket" => Ok(physical_key(0x1E, ']')),
        "equal" => Ok(physical_key(0x18, '=')),
        "minus" => Ok(physical_key(0x1B, '-')),
        "comma" => Ok(physical_key(0x2B, ',')),
        "period" => Ok(physical_key(0x2F, '.')),
        "slash" => Ok(physical_key(0x2C, '/')),
        "backslash" => Ok(physical_key(0x2A, '\\')),
        "semicolon" => Ok(physical_key(0x29, ';')),
        "apostrophe" => Ok(physical_key(0x27, '\'')),
        "grave_accent" => Ok(physical_key(0x32, '`')),

        // Single character: treat as Unicode
        s if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            Ok(Key::Unicode(c))
        }

        _ => Err(format!("Unknown key: '{}'", name)),
    }
}

/// List of recognized modifier key names for validation.
const MODIFIER_NAMES: &[&str] = &["super", "ctrl", "control", "shift", "alt"];

/// Parses a keybinding trigger string (e.g. "super+shift+d") into a `KeyCombo`.
pub fn parse_key_combo(trigger: &str) -> Result<KeyCombo, String> {
    if trigger.is_empty() {
        return Err("Empty keybinding".to_string());
    }
    let parts: Vec<&str> = trigger.split('+').collect();

    // Last element is the main key, rest are modifiers
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_key_combo_valid_cases() {
        let cases = [
            // Single modifier + character key
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
            // Multiple modifiers
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
                    key: physical_key(0x18, '='),
                },
            ),
            // No modifiers
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
            // Symbol keys
            (
                "super+ctrl+right_bracket",
                KeyCombo {
                    modifiers: vec![Key::Meta, Key::Control],
                    key: physical_key(0x1E, ']'),
                },
            ),
            (
                "super+ctrl+left_bracket",
                KeyCombo {
                    modifiers: vec![Key::Meta, Key::Control],
                    key: physical_key(0x21, '['),
                },
            ),
            // Arrow keys
            (
                "ctrl+up",
                KeyCombo {
                    modifiers: vec![Key::Control],
                    key: Key::UpArrow,
                },
            ),
            // control alias
            (
                "control+d",
                KeyCombo {
                    modifiers: vec![Key::Control],
                    key: Key::Unicode('d'),
                },
            ),
            // enter alias
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
            // Modifier keys
            ("super", Key::Meta),
            ("ctrl", Key::Control),
            ("control", Key::Control),
            ("shift", Key::Shift),
            ("alt", Key::Alt),
            // Special keys
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
            // Arrow keys
            ("up", Key::UpArrow),
            ("down", Key::DownArrow),
            ("left", Key::LeftArrow),
            ("right", Key::RightArrow),
            // Function keys
            ("f1", Key::F1),
            ("f12", Key::F12),
            // Symbol keys
            ("left_bracket", physical_key(0x21, '[')),
            ("right_bracket", physical_key(0x1E, ']')),
            ("equal", physical_key(0x18, '=')),
            ("minus", physical_key(0x1B, '-')),
            ("comma", physical_key(0x2B, ',')),
            ("period", physical_key(0x2F, '.')),
            ("slash", physical_key(0x2C, '/')),
            ("backslash", physical_key(0x2A, '\\')),
            ("semicolon", physical_key(0x29, ';')),
            ("apostrophe", physical_key(0x27, '\'')),
            ("grave_accent", physical_key(0x32, '`')),
            // Single character
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
