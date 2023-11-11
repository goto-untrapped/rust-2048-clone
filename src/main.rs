// ウィンドウの表示や操作ができる
use piston_window::*;

mod app;
mod board;
mod number_renderer;
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

    app.load();

    // オブジェクトを描画できるオブジェクトを生成
    let mut gl = GlGraphics::new(OpenGL::V3_2);

    while let Some(e) = window.next() {
        // 描画する要素を更新
        if let Some(ref args) = e.render_args() {
            app.render(args, &mut gl);
        }

        // 変数の状態を更新
        if let Some(ref args) = e.update_args() {
            app.update(args);
        }

        // キーボード入力受付
        if let Some(ref args) = e.press_args() {
            app.key_press(args);
        }
    }

}
