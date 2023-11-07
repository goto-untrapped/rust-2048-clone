use std::{env::current_exe, path::Path, fs::File, io::{BufReader, BufWriter, Write}};
use rustc_serialize::{ json, Encodable, Decodable };

static SETTING_FILENAME: &'static str = "settings.json";

pub struct Settings {
    pub board_padding: f64,
    pub board_size: [f64; 2],
    pub board_offset_y: f64,
    pub window_size: [u32; 2],
    pub tile_width: i32,
    pub tile_height: i32,
    pub tile_size: f64,
    pub tile_padding: f64,
    pub tile_move_time: f64,
    pub tile_new_time: f64,
}

impl Settings {
    pub fn load() -> Settings {
        Settings::from_settings_injson(&SettingsInJson::load())
    }

    fn from_settings_injson<'a>(s: &'a SettingsInJson) -> Settings {
        let board_size = [
            s.tile_size * s.tile_width as f64 + s.tile_padding * (s.tile_width + 1) as f64,
            s.tile_size * s.tile_height as f64 + s.tile_padding * (s.tile_height + 1) as f64,
        ];

        Settings { 
            window_size: [
                (s.board_padding * 2.0 + board_size[0]) as u32,
                (s.board_padding * 2.0 + board_size[1] + s.board_offset_y) as u32,
            ],
            board_padding: s.board_padding,
            board_size: board_size,
            board_offset_y: s.board_offset_y,
            tile_width: s.tile_width,
            tile_height: s.tile_height,
            tile_size: s.tile_size,
            tile_padding: s.tile_padding,
            tile_move_time: s.tile_move_time,
            tile_new_time: s.tile_new_time,
        }
    }
}

#[derive(RustcEncodable, RustcDecodable)]
struct SettingsInJson {
    board_padding: f64,
    board_offset_y: f64,

    tile_width: i32,
    tile_height: i32,
    tile_size: f64,
    tile_padding: f64,
    tile_move_time: f64,
    tile_new_time: f64,
}

impl SettingsInJson {
    pub fn default_settings() -> SettingsInJson {
        SettingsInJson {
            board_padding: 12.0,
            board_offset_y: 128.0,
            tile_width: 4,
            tile_height: 4,
            tile_size: 72.0,
            tile_padding: 16.0,
            tile_move_time: 0.1,
            tile_new_time: 0.1,
        }
    }

    pub fn load() -> SettingsInJson {
        // 読み取り可能なファイルパスをResult型で返す
        let exe_path = current_exe();

        // 読み取り可能なファイルパスの取得エラー時はデフォルトの設定を使う
        if exe_path.is_err() {
            return SettingsInJson::default_settings();
        }

        // 読み取り可能なファイルパスを取得
        let mut exe_path = exe_path.unwrap();
        // 読み取り可能なファイルパスを親のパスに変更
        exe_path.pop();
        // 読み取り可能なファイルパスのファイル名を設定
        let path = exe_path.join(Path::new(SETTING_FILENAME));

        // ファイルの読み込み準備
        let file = File::open(&path);

        // ファイルが存在しない場合、デフォルト設定を返す。ファイルとして作成しておく
        match file {
            Err(e) => {
                println!("Configuration file can't be open ({}). Try to generate a default one.", e);
                let default = SettingsInJson::default_settings();
                default.save();
                return default;
            },
            _ => {}
        }

        // BufferReaderを使う準備
        let mut reader = BufReader::new(file.unwrap());

        // ファイルの中身をデシリアライズするためのデコーダを準備
        let mut decoder = json::Decoder::new(json::Json::from_reader(&mut reader).unwrap());
        // デコードする
        Decodable::decode(&mut decoder).unwrap()
    }

    pub fn save(&self) {
        // 書き込み可能なファイルパスをResult型で返す
        let exe_path = current_exe();

        // 書き込み可能なファイルパスの取得エラー時は、何もしない
        if exe_path.is_err() {
            println!("WARNING: Failed to save settings: can't find exe path.");
            return;
        }

        // 書き込み可能なファイルを用意して、書き込み準備
        let path = exe_path.unwrap();
        let file = File::create(&path.with_file_name(SETTING_FILENAME)).unwrap();
        let mut writer = BufWriter::new(file);

        // 設定ファイルへ書き込み
        // SettingsInJsonが正しくエンコードできるフォーマットかどうか
        match json::encode(self) {
            // 正しくエンコードできる場合
            Ok(encoded) => {
                // ファイル書き込みにエラーが起きた場合
                if let Err(e) = writer.write(encoded.as_bytes()) {
                    println!("WARNING: Failed to save settings: {}", e);
                }
            },
            // 正しくエンコードできない場合
            Err(e) => {
                println!("WARNING: Failed to save settings: {}", e);
            }
        }
    }
}