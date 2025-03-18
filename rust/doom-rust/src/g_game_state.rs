use sdl2::{self, TimerSubsystem};

pub struct GameStateT {
    pub scrn_w: u32,
    pub scrn_h: u32,
    pub target_fps: f64,
    pub target_frame_time: f64,
    pub delta_time: f64,
    pub is_running: bool,
    pub is_paused: bool,
    pub is_fps_capped: bool,
    pub state_show_map: bool,
    pub is_debug_mode: bool,
}

pub static mut FRAME_START: u32 = 0;

pub fn g_init(scrnw: u32, scrnh: u32, target_fps: i32) -> GameStateT {
    let game_state = GameStateT {
        scrn_w: scrnw,
        scrn_h: scrnh,
        target_fps: target_fps as f64,
        target_frame_time: 1.0 / target_fps as f64,
        delta_time: 1.0 / target_fps as f64,
        is_running: true,
        is_paused: false,
        is_fps_capped: false,
        state_show_map: false,
        is_debug_mode: false,
    };
    game_state
}

pub fn g_frame_start(timer_subsystem_context: &TimerSubsystem) {
    unsafe {
        FRAME_START = TimerSubsystem::ticks(timer_subsystem_context);
    }
}

pub fn g_frame_end(state: &mut GameStateT, timer_subsystem_context: &TimerSubsystem) {
    unsafe {
        let delta_time = (TimerSubsystem::ticks(timer_subsystem_context) - FRAME_START) as f64 / 1000.0;
        state.delta_time = delta_time;

        if state.delta_time < state.target_frame_time {
            std::thread::sleep(std::time::Duration::from_secs_f64(state.target_frame_time - state.delta_time));
            state.delta_time = state.target_frame_time;
        }
    }
}