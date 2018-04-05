use std::clone::Clone;

#[derive(Clone)]
enum KeyState {
    Pressed,
    Released,
}

#[derive(Clone)]
struct KeyboardState {
    a: KeyState,
}

#[derive(Clone)]
pub struct GameState {
    keyboard_state: KeyboardState,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            keyboard_state: KeyboardState {
                a: KeyState::Released,
            },
        }
    }
}
