use std::clone::Clone;
use std::sync::RwLock;
use std::sync::Arc;

#[derive(Clone)]
enum KeyState<'a> {
    Pressed<'a>,
    Released,
}

#[derive(Clone)]
struct KeyboardState<'a> {
    a: KeyState<'a>,
}

#[derive(Clone)]
pub struct GameState<'a> {
    pub test_var: i32,
    pub keyboard_state: KeyboardState<'a>,
}

impl<'a> GameState<'a> {
    pub fn new() -> Arc<RwLock<GameState<'a>>> {
        Arc::new(RwLock::new(GameState {
            test_var: 0,
            keyboard_state: KeyboardState {
                a: KeyState::Released,
            },
        }))
    }
}
