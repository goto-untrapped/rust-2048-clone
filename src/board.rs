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
        
        board
    }
}