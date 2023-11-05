use crate::settings::Settings;
use opengl_graphics::GlGraphics;
use piston_window::*;

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

    pub fn render(&self, c: &Context, gl: &mut GlGraphics) {
        // タイルを描画
        Rectangle::new([1.0, 1.0, 1.0, 1.0]).draw(
            // TODO
            rectangle::centered([
                self.tile_x as f64 + self.settings.tile_size / 2.0,
                self.tile_y as f64 + self.settings.tile_size / 2.0,
                self.settings.tile_size as f64 / 2.0,
                self.settings.tile_size as f64 / 2.0,
            ]),
            &DrawState::default(),
            c.transform,
            gl,
        );
    }
}
