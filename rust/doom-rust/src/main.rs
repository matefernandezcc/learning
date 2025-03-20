use sdl2::{Sdl, VideoSubsystem, TimerSubsystem};
mod typedefs; mod p_player; mod g_game_state;

///////////////////////////////// SDL Contextos /////////////////////////////////
pub struct SdlContextWrapper {
    pub sdl_context: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub timer_subsystem: TimerSubsystem,
}

impl SdlContextWrapper {
    pub fn init() -> Result<Self, String> {
        let sdl_context: Sdl = sdl2::init()?;
        let video_subsystem: VideoSubsystem = sdl_context.video()?;
        let timer_subsystem: TimerSubsystem = sdl_context.timer()?;

        Ok(SdlContextWrapper {
            sdl_context,
            video_subsystem,
            timer_subsystem,
        })
    }
}

/* 
fn print_sdl_info(sdl_context: &Sdl) {
    // Obtén el subsistema de video
    let video_subsystem = sdl_context.video();
    match video_subsystem {
        Ok(video) => {
            // Aquí puedes imprimir algo relacionado con el subsistema de video.
            println!("Video Subsystem Initialized: {:?}", video);
        },
        Err(e) => println!("Failed to initialize Video subsystem: {}", e),
    }
    
    // Para otros subsistemas, puedes hacer lo mismo:
    let timer_subsystem = sdl_context.timer();
    match timer_subsystem {
        Ok(timer) => println!("Timer Subsystem Initialized: {:?}", timer),
        Err(e) => println!("Failed to initialize Timer subsystem: {}", e),
    }
}
*/


///////////////////////////////// MAIN /////////////////////////////////
fn game_loop() {
    println!("SDL2");
}
fn main() {
    // Iniciar instancias de SDL (para usar la biblioteca)
    let sdl_wrapper: SdlContextWrapper = SdlContextWrapper::init().unwrap();
    
    // init engine
    // keyboard IO
    // events handler
    // renderer
    // player

    // rendering context (window)
    game_loop();
}