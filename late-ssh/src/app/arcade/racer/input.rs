use std::time::Instant;

use super::state::{PlayerInput, State};

fn set_input(state: &mut State, input: PlayerInput) {
    state.input = input;
    state.input_last_set = Some(Instant::now());
}

pub fn handle_key(state: &mut State, byte: u8) -> bool {
    match byte {
        b'w' | b'W' => { set_input(state, PlayerInput::Accelerate); true }
        b's' | b'S' => { set_input(state, PlayerInput::Brake); true }
        b'a' | b'A' => { state.switch_lane(); true }
        b'd' | b'D' => { state.switch_lane(); true }
        b' '        => { set_input(state, PlayerInput::Handbrake); true }
        b'p' | b'P' => { state.toggle_pause(); true }
        b'r' | b'R' => { state.restart(); true }
        _ => false,
    }
}

pub fn handle_arrow(state: &mut State, key: u8) -> bool {
    match key {
        b'A' => { set_input(state, PlayerInput::Accelerate); true }
        b'B' => { set_input(state, PlayerInput::Brake); true }
        b'C' | b'D' => { state.switch_lane(); true }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::arcade::racer::state::{Lane, State};

    #[test]
    fn w_sets_accelerate() {
        let mut s = State::new();
        s.ai_cars.clear();
        handle_key(&mut s, b'w');
        assert!(matches!(s.input, PlayerInput::Accelerate));
    }

    #[test]
    fn s_sets_brake() {
        let mut s = State::new();
        s.ai_cars.clear();
        handle_key(&mut s, b's');
        assert!(matches!(s.input, PlayerInput::Brake));
    }

    #[test]
    fn a_switches_lane() {
        let mut s = State::new();
        s.ai_cars.clear();
        let original = s.player_lane;
        handle_key(&mut s, b'a');
        assert_ne!(s.player_lane, original);
    }

    #[test]
    fn up_arrow_sets_accelerate() {
        let mut s = State::new();
        s.ai_cars.clear();
        handle_arrow(&mut s, b'A');
        assert!(matches!(s.input, PlayerInput::Accelerate));
    }

    #[test]
    fn space_sets_handbrake() {
        let mut s = State::new();
        s.ai_cars.clear();
        handle_key(&mut s, b' ');
        assert!(matches!(s.input, PlayerInput::Handbrake));
    }
}
