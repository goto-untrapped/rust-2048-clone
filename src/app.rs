use opengl_graphics::GlGraphics;
use piston_window::*;
use crate::{board::Board, settings::Settings};

pub struct App<'a> {
    board: Board<'a>,
    settings: &'a Settings,

    window_background_color: [f32; 4],
}

impl<'a> App<'a> {
    pub fn new(settings: &'a Settings) -> App {
        App {
            board: Board::new(settings),
            settings: settings,

            window_background_color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        // レンダリングエリアの取得
        let area = args.window_size;
        // レンダリングエリアからコンテキストを生成
        let c = &Context::new_abs(area[0], area[1]);

        // オブジェクトを描画
        gl.draw(args.viewport(), |_, gl| {
            self.board.render(c, gl);
        })
    }

}