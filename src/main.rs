// ウィンドウの表示や操作ができる
use piston_window::*;

mod app;
mod board;
mod settings;
mod tile;

fn main() {
    // オブジェクトを描画できる
    use opengl_graphics::GlGraphics;
    // 設定をロード
    let settings = settings::Settings::load();
    // ウィンドウサイズを設定
    let (width, height) = (settings.window_size[0], settings.window_size[1]);
    // ウィンドウを初期化
    let mut window: PistonWindow = 
        WindowSettings::new("title" , [width, height])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut app = app::App::new(&settings);

    // オブジェクトを描画できるオブジェクトを生成
    let mut gl = GlGraphics::new(OpenGL::V3_2);

    let mut tile: Tile = Tile { pos_x: 100, pos_y: 100 };

    while let Some(e) = window.next() {
        // レンダリング用の引数？
        if let Some(ref args) = e.render_args() {
            app.render(args, &mut gl);
        }

        if let Some(ref args) = e.press_args() {
            app.key_press(args);
        }
    }

    // タイルを右に動かす
    fn move_tile_to_right(tile: &mut Tile) {
        tile.pos_x = 200;
        println!("{}", tile.pos_x);
    }
}

struct Tile {
    pos_x: i32,
    pos_y: i32,
}