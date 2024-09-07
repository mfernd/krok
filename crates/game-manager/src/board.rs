use crate::player::PlayerIdentifier;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Board {
    layout: BoardLayout,
    player_positions: HashMap<PlayerIdentifier, BoardPosition>,
}

impl Board {
    pub fn new_with_first_player(layout: BoardLayout, first_player_id: PlayerIdentifier) -> Self {
        Self {
            layout,
            player_positions: HashMap::from([(first_player_id, BoardPosition::new(0, 0))]),
        }
    }
}

#[derive(Debug)]
pub enum BoardLayout {
    Classic,
}

#[derive(Debug)]
pub struct BoardPosition {
    x: u8,
    y: u8,
}

impl BoardPosition {
    // TODO should change to take into account the board layout?
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}
