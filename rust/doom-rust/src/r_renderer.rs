use sdl2;
mod typedefs;
mod p_player;
mod g_game_state;
mod u_utils;

///////////////////////////////// Structs necesarios /////////////////////////////////
pub struct RPlaneT {
    pub t: [i32; 1024],
    pub b: [i32; 1024],
}

pub struct WallT {
    pub a: vec2,
    pub b: vec2,
    pub portal_top_height: f64,
    pub portal_bot_height: f64,
    pub is_portal: bool,
}

pub struct SectorT {
    pub id: i32,
    pub walls: [Wall; 10],
    pub num_walls: i32,
    pub height: i32,
    pub elevation: i32,
    pub dist: f64,
    pub color: u32,
    pub floor_clr: u32,
    pub ceil_clr: u32,

    pub portals_floorx_ylut: R_plane,
    pub portals_ceilx_ylut: R_plane,
    pub floorx_ylut: R_plane,
    pub ceilx_ylut: R_plane,
}

pub struct SectorsQueueT {
    pub sectors: [Sector; 1024],
    pub num_sectors: i32,
}

///////////////////////////////// Implementaciones y funciones del Render /////////////////////////////////
pub fn r_init(main_win: &SDL_Window, game_state: &GameStateT);
pub fn r_shutdown();
pub fn r_render(player: &PlayerT, game_state: &GameStateT);
pub fn r_draw_walls(player: &PlayerT, game_state: &GameStateT);
pub fn r_create_sector(height: i32, elevation: i32, color: u32, ceil_clr: u32, floor_clr: u32) -> SectorT;
pub fn r_sector_add_wall(sector: &SectorT, vertices: WallT);
pub fn r_add_sector_to_queue(sector: &SectorT);
pub fn r_create_wall(ax: i32, ay: i32, bx: i32, by: i32) -> WallT;
pub fn r_create_portal(ax: i32, ay: i32, bx: i32, by: i32, th:i32, bh:i32) -> WallT;