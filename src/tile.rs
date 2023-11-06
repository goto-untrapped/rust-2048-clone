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
        // タイルの座標を計算
        let mut pos = self.tile_to_pos(self.tile_x, self.tile_y);
        // タイルのサイズ
        let mut size = (self.settings.tile_size, self.settings.tile_size);

        let (x, y) = pos;
        let (w, h) = size;

        // タイルを描画
        Rectangle::new([1.0, 1.0, 1.0, 1.0]).draw(
            // TODO
            rectangle::centered([
                x as f64 + self.settings.tile_size / 2.0,
                y as f64 + self.settings.tile_size / 2.0,
                w as f64 / 2.0,
                h as f64 / 2.0,
            ]),
            &DrawState::default(),
            c.transform,
            gl,
        );
    }

    fn tile_to_pos(&self, tile_x: i32, tile_y: i32) -> (f64, f64) {
        let x = self.settings.board_padding + tile_x as f64 * self.settings.tile_size + (tile_x + 1) as f64 * self.settings.tile_padding;
        let y = self.settings.board_padding + self.settings.board_offset_y + tile_y as f64 * self.settings.tile_size + (tile_y + 1) as f64 * self.settings.tile_padding;
        (x, y)
    }
}
