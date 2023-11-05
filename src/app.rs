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
}