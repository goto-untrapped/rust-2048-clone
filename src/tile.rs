use crate::settings::Settings;

pub struct Tile<'a> {
    pub tile_x: i32,
    pub tile_y: i32,

    settings: &'a Settings,
}

impl<'a> Tile<'a> {
    pub fn new(settings: &'a Settings, tile_x: i32, tile_y: i32) -> Tile<'a> {
        Tile {
            tile_x: tile_x,
            tile_y: tile_y,

            settings: settings,
        }
    }
}