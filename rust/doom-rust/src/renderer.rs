use sdl2::{video::Window, VideoSubsystem, render::{Canvas, Texture}, pixels::PixelFormatEnum, Sdl};
use crate::{game_state, main, player, typedefs, utils, window};


///////////////////////////////// STRUCTS /////////////////////////////////
pub struct RPlaneT {
    pub t: [i32; 1024],
    pub b: [i32; 1024],
}

pub struct WallT {
    pub a: typedefs::Vec2T,
    pub b: typedefs::Vec2T,
    pub portal_top_height: f64,
    pub portal_bot_height: f64,
    pub is_portal: bool,
}

pub struct SectorT {
    pub id: i32,
    pub walls: [WallT; 10],
    pub num_walls: i32,
    pub height: i32,
    pub elevation: i32,
    pub dist: f64,
    pub color: u32,
    pub floor_clr: u32,
    pub ceil_clr: u32,

    pub portals_floorx_ylut: RPlaneT,
    pub portals_ceilx_ylut: RPlaneT,
    pub floorx_ylut: RPlaneT,
    pub ceilx_ylut: RPlaneT,
}

pub struct SectorsQueueT {
    pub sectors: [SectorT; 1024],
    pub num_sectors: i32,
}

pub struct Screen {
    pub screen_buffer: Vec<u32>,
    pub screen_texture: Option<Texture>,
    pub screen_buffer_size: usize,
}

impl Screen {
    pub fn init_screen(&mut self, sdl_context: &Sdl, w: u32, h: u32) {
        self.screen_buffer_size = (w * h) as usize;
        self.screen_buffer = vec![0; self.screen_buffer_size];
        
        // Obtén el VideoSubsystem
        let video_subsystem = sdl_context.video().unwrap();
    
        let window = video_subsystem.window("Window", w, h).position_centered().build().unwrap();
        let mut canvas = window.into_canvas().accelerated().build().unwrap();
        
        // Crear la textura para la pantalla
        match canvas.create_texture_streaming(PixelFormatEnum::RGBA32, w, h) {
            Ok(texture) => {
                self.screen_texture = Some(texture);
            }
            Err(e) => {
                eprintln!("Error creating screen texture: {}", e);
                self.shutdown();
            }
        }
    }
    fn shutdown(&self) {
        eprintln!("Shutting down screen resources.");
    }
}

///////////////////////////////// FUNCIONES  /////////////////////////////////
pub fn r_init(main_win: Window, game_state: &game_state::GameStateT) {
    let scrnw: u32 = game_state.scrn_w;
    let scrnh: u32 = game_state.scrn_h;

    // Crear el Renderer
    let canvas = main_win
        .into_canvas() // Convierte la ventana en un canvas
        .accelerated()  // Habilita aceleración por hardware
        .build()// Crea el renderer
        .unwrap();
}

pub fn r_shutdown(){}

pub fn r_render(player: &player::PlayerT, game_state: &game_state::GameStateT){

}

pub fn r_draw_walls(player: &player::PlayerT, game_state: &game_state::GameStateT){}

pub fn r_create_sector(height: i32, elevation: i32, color: u32, ceil_clr: u32, floor_clr: u32) -> SectorT{}

pub fn r_sector_add_wall(sector: &SectorT, vertices: WallT){}

pub fn r_add_sector_to_queue(sector: &SectorT){}

pub fn r_create_wall(ax: i32, ay: i32, bx: i32, by: i32) -> WallT{}

pub fn r_create_portal(ax: i32, ay: i32, bx: i32, by: i32, th:i32, bh:i32) -> WallT{}