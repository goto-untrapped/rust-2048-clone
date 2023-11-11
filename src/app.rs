use opengl_graphics::GlGraphics;
use piston_window::*;
use crate::{board::Board, settings::Settings, number_renderer::NumberRenderer};

pub struct App<'a> {
    board: Board<'a>,
    number_renderer: Option<NumberRenderer>,
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

    pub fn update(&mut self, args: &UpdateArgs) {
        self.board.update(args.dt);
    }

    pub fn key_press(&mut self, args: &Button) {
        use piston_window::Button::Keyboard;

        if *args == Keyboard(Key::Right) {
            self.board.merge_from_left_to_right();
        }

        if *args == Keyboard(Key::Left) {
            self.board.merge_from_right_to_left();
        }
    }

}