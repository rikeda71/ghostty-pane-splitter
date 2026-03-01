use enigo::{Direction, Enigo, Keyboard, Settings};
use std::thread;
use std::time::Duration;

use crate::config::Keybindings;
use crate::keybind::KeyCombo;
use crate::layout::Layout;

/// Delay in milliseconds between split operations to allow Ghostty to create new panes.
const SPLIT_DELAY_MS: u64 = 200;
/// Delay in milliseconds between navigation operations (focus move only).
const NAV_DELAY_MS: u64 = 50;
/// Delay in milliseconds for rapid consecutive focus moves (Phase 5).
const FAST_NAV_DELAY_MS: u64 = 20;

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
    let split_delay = Duration::from_millis(SPLIT_DELAY_MS);
    let nav_delay = Duration::from_millis(NAV_DELAY_MS);
    let fast_nav_delay = Duration::from_millis(FAST_NAV_DELAY_MS);

    let num_cols = layout.num_cols();

    // Phase 1: Create columns (horizontal splits)
    for _ in 0..(num_cols - 1) {
        press_key_combo(&mut enigo, &keybindings.split_right)
            .map_err(|e| format!("Failed to send split_right: {}", e))?;
        thread::sleep(split_delay);
    }

    // Phase 2: Navigate back to first column
    for _ in 0..(num_cols - 1) {
        press_key_combo(&mut enigo, &keybindings.goto_previous)
            .map_err(|e| format!("Failed to send goto_previous: {}", e))?;
        thread::sleep(nav_delay);
    }

    // Phase 3: Create rows in each column (vertical splits)
    for col in 0..num_cols {
        for _ in 0..(layout.columns[col] - 1) {
            press_key_combo(&mut enigo, &keybindings.split_down)
                .map_err(|e| format!("Failed to send split_down: {}", e))?;
            thread::sleep(split_delay);
        }
        if col < num_cols - 1 {
            press_key_combo(&mut enigo, &keybindings.goto_next)
                .map_err(|e| format!("Failed to send goto_next: {}", e))?;
            thread::sleep(nav_delay);
        }
    }

    // Phase 4: Equalize pane sizes
    press_key_combo(&mut enigo, &keybindings.equalize)
        .map_err(|e| format!("Failed to send equalize: {}", e))?;
    thread::sleep(split_delay);

    // Phase 5: Navigate to top-left pane
    for _ in 0..(layout.total_panes() - 1) {
        press_key_combo(&mut enigo, &keybindings.goto_previous)
            .map_err(|e| format!("Failed to send goto_previous: {}", e))?;
        thread::sleep(fast_nav_delay);
    }

    Ok(())
}
