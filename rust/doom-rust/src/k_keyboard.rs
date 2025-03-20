use sdl2::{event::EventType, keyboard::Scancode, EventPump, Sdl};
use crate::{g_game_state, p_player};

// Velocidades predeterminadas
pub struct SpeedT {
    pub mov_speed: i32,
    pub elevation_speed: i32,
    pub rot_speed: i32,
}
impl SpeedT {
    pub fn new() -> Self {
        SpeedT {
            mov_speed: 150,
            elevation_speed: 500*100,
            rot_speed: 4,
        }
    }
}
///////////////////////////////// STRUCTS /////////////////////////////////
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

enum KbdKeyState {
    Up,
    Down,
}


///////////////////////////////// FUNCIONES /////////////////////////////////
// Mapeo de teclas
pub fn k_init_keymap(keymap: &mut KeymapT, keystates: &mut KeystatesT) {
    *keymap = KeymapT {
        forward: Scancode::W,
        backward: Scancode::S,
        left: Scancode::A,
        right: Scancode::D,
        quit: Scancode::Escape,
        strafe_left: Scancode::Q,
        strafe_right: Scancode::E,
        up: Scancode::Space,
        down: Scancode::LCtrl,
        toggle_map: Scancode::M,
        debug_mode: Scancode::O,
    };

    *keystates = KeystatesT {
        forward: false,
        backward: false,
        left: false,
        right: false,
        strafe_left: false,
        strafe_right: false,
        up: false,
        down: false,
        quit: false,
        toggle_map: false,
        debug_mode: false,
    };
}

// Manejar Eventos del teclado
pub fn k_handle_events(event_pump: &mut EventPump, game_state: &g_game_state::GameStateT, player: &p_player::PlayerT) {

}

// Procesar el estado de las teclas (actualiza el estado de 'KeystatesT')
pub fn k_process_keystates() {
}

// Manejar las teclas en tiempo real (presionar o soltar)
pub fn k_handle_realtimekeys(key_scancode: &Scancode, state: KbdKeyState){
}