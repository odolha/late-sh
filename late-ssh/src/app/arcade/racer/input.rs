//! Key input dispatch for the Racer game.
//!
//! Two surfaces share one state: the picker (track selection) and the
//! racing view. The dispatcher routes to the right handler based on
//! `state.screen`.

use super::state::{PlayerInput, RacerScreen, State};

// ─── Picker ─────────────────────────────────────────────────────────────────

fn handle_picker_key(state: &mut State, byte: u8) -> bool {
    match byte {
        b'j' | b'J' => {
            state.picker_move(1);
            true
        }
        b'k' | b'K' => {
            state.picker_move(-1);
            true
        }
        b'\r' | b'\n' | b' ' => {
            state.start_selected_track();
            true
        }
        _ => false,
    }
}

fn handle_picker_arrow(state: &mut State, key: u8) -> bool {
    match key {
        b'A' => {
            state.picker_move(-1);
            true
        }
        b'B' => {
            state.picker_move(1);
            true
        }
        _ => false,
    }
}

// ─── Racing ─────────────────────────────────────────────────────────────────

fn handle_race_key(state: &mut State, byte: u8) -> bool {
    match byte {
        b'w' | b'W' => {
            state.set_input(PlayerInput::Accelerate);
            true
        }
        b's' | b'S' => {
            state.set_input(PlayerInput::Brake);
            true
        }
        b'a' | b'A' => {
            state.move_left();
            true
        }
        b'd' | b'D' => {
            state.move_right();
            true
        }
        b' ' => {
            state.set_input(PlayerInput::Handbrake);
            true
        }
        b'p' | b'P' => {
            state.toggle_pause();
            true
        }
        b'r' | b'R' => {
            state.restart_current();
            true
        }
        b't' | b'T' => {
            state.return_to_picker();
            true
        }
        _ => false,
    }
}

fn handle_race_arrow(state: &mut State, key: u8) -> bool {
    match key {
        b'A' => {
            state.set_input(PlayerInput::Accelerate);
            true
        }
        b'B' => {
            state.set_input(PlayerInput::Brake);
            true
        }
        b'C' => {
            state.move_right();
            true
        }
        b'D' => {
            state.move_left();
            true
        }
        _ => false,
    }
}

// ─── Dispatchers ───────────────────────────────────────────────────────────

pub fn handle_key(state: &mut State, byte: u8) -> bool {
    match state.screen {
        RacerScreen::Picker => handle_picker_key(state, byte),
        RacerScreen::Racing => handle_race_key(state, byte),
    }
}

pub fn handle_arrow(state: &mut State, key: u8) -> bool {
    match state.screen {
        RacerScreen::Picker => handle_picker_arrow(state, key),
        RacerScreen::Racing => handle_race_arrow(state, key),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::arcade::racer::tracks::DEFAULT_TRACK;

    #[test]
    fn picker_enter_starts_a_track() {
        let mut s = State::new();
        handle_key(&mut s, b'\r');
        assert_eq!(s.screen, RacerScreen::Racing);
    }

    #[test]
    fn race_w_sets_accelerate() {
        let mut s = State::new();
        s.start_track(DEFAULT_TRACK);
        handle_key(&mut s, b'w');
        assert!(matches!(s.input, PlayerInput::Accelerate));
    }
}
