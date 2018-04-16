use std::clone::Clone;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Clone, Debug)]
pub enum KeyState {
    Pressed,
    Released,
}

#[derive(Clone, Debug)]
pub struct KeyboardState {
    pub key_map: HashMap<&'static str, KeyState>,
}

impl KeyboardState {
    fn init() -> KeyboardState {
        let mut state = KeyboardState {
            key_map: HashMap::new(),
        };
        state.key_map.insert("key_a", KeyState::Released);
        state.key_map.insert("key_b", KeyState::Released);
        state.key_map.insert("key_c", KeyState::Released);
        state.key_map.insert("key_d", KeyState::Released);
        state.key_map.insert("key_e", KeyState::Released);
        state.key_map.insert("key_f", KeyState::Released);
        state.key_map.insert("key_g", KeyState::Released);
        state.key_map.insert("key_h", KeyState::Released);
        state.key_map.insert("key_i", KeyState::Released);
        state.key_map.insert("key_j", KeyState::Released);
        state.key_map.insert("key_k", KeyState::Released);
        state.key_map.insert("key_l", KeyState::Released);
        state.key_map.insert("key_m", KeyState::Released);
        state.key_map.insert("key_n", KeyState::Released);
        state.key_map.insert("key_o", KeyState::Released);
        state.key_map.insert("key_p", KeyState::Released);
        state.key_map.insert("key_q", KeyState::Released);
        state.key_map.insert("key_r", KeyState::Released);
        state.key_map.insert("key_s", KeyState::Released);
        state.key_map.insert("key_t", KeyState::Released);
        state.key_map.insert("key_u", KeyState::Released);
        state.key_map.insert("key_v", KeyState::Released);
        state.key_map.insert("key_w", KeyState::Released);
        state.key_map.insert("key_x", KeyState::Released);
        state.key_map.insert("key_y", KeyState::Released);
        state.key_map.insert("key_z", KeyState::Released);
        state.key_map.insert("key_1", KeyState::Released);
        state.key_map.insert("key_2", KeyState::Released);
        state.key_map.insert("key_3", KeyState::Released);
        state.key_map.insert("key_4", KeyState::Released);
        state.key_map.insert("key_5", KeyState::Released);
        state.key_map.insert("key_6", KeyState::Released);
        state.key_map.insert("key_7", KeyState::Released);
        state.key_map.insert("key_8", KeyState::Released);
        state.key_map.insert("key_9", KeyState::Released);
        state.key_map.insert("key_0", KeyState::Released);
        state.key_map.insert("key_ctrl", KeyState::Released);
        state.key_map.insert("key_shift", KeyState::Released);
        state.key_map.insert("key_alt", KeyState::Released);
        state.key_map.insert("key_pipe", KeyState::Released);
        state.key_map.insert("key_tab", KeyState::Released);
        state.key_map.insert("key_caps", KeyState::Released);
        state.key_map.insert("key_gtlt", KeyState::Released);
        state.key_map.insert("key_up", KeyState::Released);
        state.key_map.insert("key_down", KeyState::Released);
        state.key_map.insert("key_left", KeyState::Released);
        state.key_map.insert("key_right", KeyState::Released);
        state.key_map.insert("key_home", KeyState::Released);
        state.key_map.insert("key_end", KeyState::Released);
        state.key_map.insert("key_pgup", KeyState::Released);
        state.key_map.insert("key_pgdown", KeyState::Released);
        state.key_map.insert("key_insert", KeyState::Released);
        state.key_map.insert("key_delete", KeyState::Released);
        state.key_map.insert("key_esc", KeyState::Released);
        state.key_map.insert("key_backspace", KeyState::Released);
        state.key_map.insert("key_return", KeyState::Released);
        state.key_map.insert("key_space", KeyState::Released);
        return state;
    }
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub test_var: i32,
    pub keyboard_state: KeyboardState,
}

impl GameState {
    pub fn new() -> Arc<RwLock<GameState>> {
        Arc::new(RwLock::new(GameState {
            test_var: 0,
            keyboard_state: KeyboardState::init(),
        }))
    }

    pub fn update_keyboard_state(&mut self, key: &'static str, state: KeyState) {
        if let Some(k) = self.keyboard_state.key_map.get_mut(key) {
            *k = state;
        }
    }
}
