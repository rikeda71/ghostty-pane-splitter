use enigo::Key;

#[derive(Debug, PartialEq)]
pub struct KeyCombo {
    pub modifiers: Vec<Key>,
    pub key: Key,
}

pub fn parse_ghostty_key(name: &str) -> Result<Key, String> {
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

pub fn parse_key_combo(trigger: &str) -> Result<KeyCombo, String> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
