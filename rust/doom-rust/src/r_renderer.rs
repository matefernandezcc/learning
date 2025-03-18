mod typedefs;

pub struct _r_plane {
    pub t: [i32; 1024],
    pub b: [i32; 1024],
}

pub struct _wall {
    pub a: _vec2,
    pub b: _vec2,
    pub portal_top_height: f64,
    pub portal_bot_height: f64,
    pub is_portal: bool,
}

pub struct _sector {
    pub id: i32,
    pub walls: [_wall; 10],
    pub num_walls: i32,
    pub height: i32,
    pub elevation: i32,
    pub dist: f64,
    pub color: u32,
    pub floor_clr: u32,
    pub ceil_clr: u32,

    pub portals_floorx_ylut: _r_plane,
    pub portals_ceilx_ylut: _r_plane,
    pub floorx_ylut: _r_plane,
    pub ceilx_ylut: _r_plane,
}

pub struct _sectors_queue {
    pub sectors: [_sector; 1024],
    pub num_sectors: i32,
}

pub fn R_Init();
pub fn R_Shutdown();
pub fn R_Render();
pub fn R_DrawWalls();
pub fn R_CreateSector();
