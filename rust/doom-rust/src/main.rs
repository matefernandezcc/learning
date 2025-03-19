use sdl2;
mod typedefs;
mod p_player;
mod g_game_state;

fn game_loop() {
    println!("Hello, Doom!");
}


fn main() {
    // Iniciar instancia de SDL (para usar la biblioteca)
    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
    
    // init engine
    // keyboard IO
    // events handler
    // renderer
    // player
    // rendering context (window)
    game_loop();
}

