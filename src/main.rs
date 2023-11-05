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
            // generate_board(args, &mut gl);
            // tile = generate_tile(args, &mut gl, &mut tile);
        }

        if let Some(ref args) = e.press_args() {
            use piston_window::Button::Keyboard;

            if *args == Keyboard(Key::Right) {
                println!("right entered");
                move_tile_to_right(&mut tile);
            }
        }
    }

    // ボードを描画
    fn generate_board(args: &RenderArgs, gl: &mut GlGraphics) {
        for x in 0..2 {
            for y in 0..2 {
                gl.draw(args.viewport(), |c, gl| {
                    Rectangle::new([0.0, 0.5, 0.0, 1.0])
                        .draw(rectangle::centered([100.0 * (x + 1) as f64, 100.0 * (y + 1) as f64, 50.0, 50.0]), 
                            &DrawState::default(), 
                            c.transform,   
                            gl);
                });
            }
        }
    }

    // 四角を描画
    fn generate_tile(args: &RenderArgs, gl: &mut GlGraphics, tile: &mut Tile) -> Tile {
        gl.draw(args.viewport(), |c, gl| {
            Rectangle::new([1.0, 1.0, 1.0, 1.0])
                .draw(rectangle::centered([tile.pos_x as f64, tile.pos_y as f64, 50.0, 50.0]), 
                    &DrawState::default(), 
                    c.transform,   
                    gl);
        });
        Tile { pos_x: tile.pos_x, pos_y: tile.pos_y }
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