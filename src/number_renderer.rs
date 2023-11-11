use std::path::Path;
use piston_window::*;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;

static DIGITS_WIDTH: f64 = 20.0;
static DIGITS_HEIGHT: f64 = 26.0;

pub struct NumberRenderer {
    image: GlTexture,
}

impl NumberRenderer {
    pub fn new() -> NumberRenderer {
        NumberRenderer {
            image: GlTexture::from_path(
                Path::new("bin/assets/digits.png"),
                &TextureSettings::new(),
            ).unwrap(),
        }
    }

    pub fn render(&self, number: u32, center_x: f64, center_y: f64, max_width: f64, c: &Context, gl: &mut GlGraphics) {
        // タイルのスコアから数字のVecを生成
        let digits = number_to_digits(number);
        // タイルに書き込むスコアの合計幅
        let total_width = DIGITS_WIDTH * digits.len() as f64;
        // 合計幅がタイルの横幅を超える場合の処理
        let total_width = if total_width > max_width {
            max_width
        } else {
            total_width
        };
        // 書き込む数字の幅方向の座標を、タイルの中央に合わせる
        let mut x = center_x - total_width / 2.0;
        // 数字1つあたりの幅
        let width = total_width / digits.len() as f64;
        // 高さも幅の比率に合わせる
        let height = width / DIGITS_WIDTH * DIGITS_HEIGHT;
        // 書き込む数字の高さ方向の座標を、タイルの中央に合わせる
        let y = center_y - height / 2.0;

        // 数字を書き込む
        for digit in digits.iter() {
            Image::new_color([0.0, 0.5, 0.0, 1.0])
                .src_rect([(*digit * DIGITS_WIDTH as u32) as f64, 0 as f64, DIGITS_WIDTH as f64, DIGITS_HEIGHT as f64])
                .rect([x, y, width, height])
                .draw(&self.image, &DrawState::default(), c.transform, gl);

            x += width;
        }
    }
}

fn number_to_digits(number: u32) -> Vec<u32> {
    // 数字を詰め替えるVecを定義
    let mut digits = Vec::<u32>::new();
    // ボードの初期スコアは0のため
    if number == 0 {
        digits.push(0);
        return digits;
    }
    // スコアが16なら[1, 6]になるように詰め替える
    let mut n = number;
    while n != 0 {
        digits.insert(0, n % 10);
        n /= 10;
    }

    digits
}