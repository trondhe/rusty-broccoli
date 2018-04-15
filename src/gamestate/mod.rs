use std::clone::Clone;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Clone)]
enum KeyState {
    Pressed,
    Released,
}

#[derive(Clone)]
struct KeyboardState {
    a: KeyState,
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub test_var: i32,
    //pub keyboard_state: KeyboardState,
}

impl GameState {
    pub fn new() -> Arc<RwLock<GameState>> {
        Arc::new(RwLock::new(GameState { test_var: 0 }))
    }
}
