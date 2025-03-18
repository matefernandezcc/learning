use sdl2::keyboard::Scancode;
mod g_game_state;
mod p_player;

pub struct KeymapT {
    pub left: Scancode,
    pub right: Scancode,
    pub forward: Scancode,
    pub backward: Scancode,
    pub strafe_left: Scancode,
    pub strafe_right: Scancode,
    pub up: Scancode,
    pub down: Scancode,
    pub quit: Scancode,
    pub toggle_map: Scancode,
    pub debug_mode: Scancode,
}

pub struct KeystatesT {
    pub left: bool,
    pub right: bool,
    pub forward: bool,
    pub backward: bool,
    pub strafe_left: bool,
    pub strafe_right: bool,
    pub up: bool,
    pub down: bool,
    pub quit: bool,
    pub toggle_map: bool,
    pub debug_mode: bool,
}

#[derive(Debug)] // Para imprimir el enum fácil
enum KbdKeyState {
    Up,
    Down,
}

pub fn k_init_keymap(){
    
}

pub fn k_handle_events(game_state: &GameStateT, player: &PlayerT);
pub fn k_process_keystates();
pub fn k_handle_realtimekeys(key_scancode: Scancode, state: KbdKeyState);