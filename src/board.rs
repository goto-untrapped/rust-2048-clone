use opengl_graphics::GlGraphics;
use rand::random;
use piston_window::*;
use crate::{tile::Tile, settings::Settings};

pub struct Board<'a> {
    tiles: Vec<Tile<'a>>,
    settings: &'a Settings,
}

impl<'a> Board<'a> {
    pub fn new(settings: &Settings) -> Board {
        let mut board = Board {
            tiles: Vec::<Tile>::new(),
            settings: settings,
        };
        board.generate_tile();
        board
    }

    pub fn generate_tile(&mut self) {
        // タイルの枚数が最大枚数と等しい場合、タイル生成しない

        // タイル生成が可能な場所にタイル生成
        loop {
            // ランダムでタイルのx座標、y座標を初期化
            let x = (random::<u32>() % self.settings.tile_width as u32) as i32;
            let y = (random::<u32>() % self.settings.tile_height as u32) as i32;

            // 初期化した座標にタイルがない場合
            if self.get_tile(x, y).is_none() {
                // ランダムでタイルのスコアを設定：後でやる
                
                // タイルを追加
                self.tiles.push(Tile::new(self.settings, x, y));
                break;
            }
        }
    }

    fn get_tile<'b>(&'b self, x: i32, y: i32) -> Option<&'b Tile<'a>> {
        for tile in self.tiles.iter() {
            if tile.tile_x == x && tile.tile_y == y {
                return Some(tile);
            }
        }

        None
    }

    pub fn render(&self, c: &Context, gl: &mut GlGraphics) {

        // ボードを描画
        self.render_board(c, gl);
        // タイルを描画
        self.render_tiles(c, gl);
    }

    fn render_board(&self, c: &Context, gl: &mut GlGraphics) {
        // ボードの外枠を描画
        Rectangle::new([0.0, 0.5, 0.0, 1.0])
        .draw(
            [
                self.settings.board_padding,
                self.settings.board_padding + self.settings.board_offset_y,
                self.settings.board_size[0],
                self.settings.board_size[1]
            ], 
            &DrawState::default(), 
            c.transform, 
            gl
        );
        // ボードのタイル配置場所を初期化
        let mut x = self.settings.board_padding + self.settings.tile_padding;
        let mut y = self.settings.board_padding + self.settings.board_offset_y + self.settings.tile_padding;

        // ボードのタイル配置場所を描画
        for _ in 0..self.settings.tile_height {
            for _ in 0..self.settings.tile_width {
                Rectangle::new([0.0, 0.0, 0.0, 1.0])
                .draw(
                    [x, y, self.settings.tile_size, self.settings.tile_size], 
                    &DrawState::default(), 
                    c.transform, 
                    gl);

                // 次のタイルのx座標を設定
                x += self.settings.tile_padding + self.settings.tile_size;
            }

            // 次の段のタイルの座標を設定
            x = self.settings.board_padding + self.settings.tile_padding;
            y += self.settings.tile_padding + self.settings.tile_size;
        }
    }

    fn render_tiles(&self, c: &Context, gl: &mut GlGraphics) {
        for tile in self.tiles.iter() {
            tile.render(c, gl);
        }
    }
}