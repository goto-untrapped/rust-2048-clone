use opengl_graphics::GlGraphics;
use rand::random;
use piston_window::{*, modular_index::next};
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

    pub fn update(&mut self, dt: f64) {
        for tile in self.tiles.iter_mut() {
            tile.update(dt);
        }
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

    pub fn move_from_left_to_right(&mut self) {
        let width = self.settings.tile_width;
        // 動かす先のタイルの検索に使う
        self.merge_row(width - 1, -1, -1);
    }

    pub fn move_from_right_to_left(&mut self) {
        let width = self.settings.tile_width;
        self.merge_row(0, width - 1, 1)
    }

    fn merge_row(&mut self, x_start: i32, x_end: i32, x_step: i32) {
        for tile in &self.tiles {
            println!("{:?} in merge_row_first", tile.status);
        }

        let mut need_generate = false;
        let mut steps: Vec<i32> = Vec::with_capacity(self.settings.tile_width as usize);
        let mut next_step = x_start;

        // 動かす先のタイルの検索に使う
        // 左から右に動かす時
        if x_step < 0 {
            while next_step > x_end {
                steps.push(next_step); next_step += x_step
            } 
        } // 右から左に動かす時
        else {
            while next_step < x_end {
                steps.push(next_step); next_step += x_step
            }
        }

        // タイルを動かす
        loop {
            for row in 0..self.settings.tile_height {
                for col in steps.to_vec() {
                    // 動かす先に既にタイルが置いてあるか
                    match self.get_mut_tile(col, row) {
                        // まだタイルが置かれていない場合
                        None => {
                            // 動かす元のタイルがあるか
                            match self.get_mut_next_tile(col, row, x_step, 0) {
                                // 動かす元のタイルがある場合、タイルの座標を更新して、動かす
                                Some (ref mut tile) => {
                                    println!("move ({}, {}) to ({}, {})", tile.tile_x, tile.tile_y, col, row);
                                    need_generate = true;
                                    tile.start_moving(0.1, col, row);
                                },
                                // 動かす元のタイルがない場合、何もしない
                                _ => {},
                            }
                        },
                        // タイルが既に置かれている場合、何もしないa
                        _ => {},
                    }
                }
            }
            break;
        }
    }

    fn get_mut_next_tile<'b>(&'b mut self, x: i32, y: i32, step_x: i32, step_y: i32) -> Option<&'b mut Tile<'a>> {
        let mut x = x + step_x;
        let mut y = y + step_y;
        let mut found = false;
        while x >= 0 && x < self.settings.tile_width
        && y >= 0 && y < self.settings.tile_height {
            let tile = self.get_tile(x, y);

            if tile.is_some() {
                found = true;
                break;
            }
            x += step_x;
            y += step_y;
        }

        if found {
            self.get_mut_tile(x, y)
        } else {
            None
        }
    }

    fn get_mut_tile<'b>(&'b mut self, x: i32, y: i32) -> Option<&'b mut Tile<'a>> {
        for tile in self.tiles.iter_mut() {
            if tile.tile_x == x && tile.tile_y == y {
                return Some(tile);
            }
        }

        None
    }



    
}