use enigo::{Direction, Enigo, Keyboard, Settings};
use std::thread;
use std::time::Duration;

use crate::config::Keybindings;
use crate::keybind::KeyCombo;
use crate::layout::Layout;

/// Delay in milliseconds between key operations to allow Ghostty to process each action.
const DELAY_MS: u64 = 200;

/// Sends a key combination via enigo by pressing modifiers, clicking the key, and releasing.
fn press_key_combo(enigo: &mut Enigo, combo: &KeyCombo) -> Result<(), enigo::InputError> {
    for modifier in &combo.modifiers {
        enigo.key(*modifier, Direction::Press)?;
    }
    enigo.key(combo.key, Direction::Click)?;
    for modifier in combo.modifiers.iter().rev() {
        enigo.key(*modifier, Direction::Release)?;
    }
    Ok(())
}

/// Executes pane splits by simulating keyboard input according to the given layout.
pub fn execute_splits(keybindings: &Keybindings, layout: &Layout) -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to initialize enigo: {}", e))?;
    let delay = Duration::from_millis(DELAY_MS);

    // Phase 1: 列を作成（水平分割）
    for _ in 0..(layout.cols - 1) {
        press_key_combo(&mut enigo, &keybindings.split_right)
            .map_err(|e| format!("Failed to send split_right: {}", e))?;
        thread::sleep(delay);
    }

    // 最初の列に戻る
    for _ in 0..(layout.cols - 1) {
        press_key_combo(&mut enigo, &keybindings.goto_previous)
            .map_err(|e| format!("Failed to send goto_previous: {}", e))?;
        thread::sleep(delay);
    }

    // Phase 2: 各列に行を作成（垂直分割）
    if layout.rows > 1 {
        for col in 0..layout.cols {
            for _ in 0..(layout.rows - 1) {
                press_key_combo(&mut enigo, &keybindings.split_down)
                    .map_err(|e| format!("Failed to send split_down: {}", e))?;
                thread::sleep(delay);
            }
            if col < layout.cols - 1 {
                press_key_combo(&mut enigo, &keybindings.goto_next)
                    .map_err(|e| format!("Failed to send goto_next: {}", e))?;
                thread::sleep(delay);
            }
        }
    }

    // Phase 3: pane サイズの均等化
    press_key_combo(&mut enigo, &keybindings.equalize)
        .map_err(|e| format!("Failed to send equalize: {}", e))?;
    thread::sleep(delay);

    Ok(())
}
