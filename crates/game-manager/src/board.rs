use crate::player::PlayerIdentifier;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Board {
    layout: BoardLayout,
    player_positions: HashMap<PlayerIdentifier, BoardPosition>,
}

impl Board {
    pub fn new_with_first_player(layout: BoardLayout, first_player_id: PlayerIdentifier) -> Self {
        Self {
            layout,
            player_positions: HashMap::from([(first_player_id, BoardPosition::first_position())]),
        }
    }

    pub fn add_player_at_the_start(
        &mut self,
        player_id: PlayerIdentifier,
    ) -> Result<(), BoardError> {
        self.player_positions
            .insert(player_id, BoardPosition::first_position())
            .ok_or(BoardError::CouldNotPlacePlayerOnBoard)?;

        Ok(())
    }

    pub fn remove_player(&mut self, player_id: &PlayerIdentifier) -> Result<(), BoardError> {
        self.player_positions
            .remove(player_id)
            .ok_or(BoardError::CouldNotRemovePlayerFromBoard)?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum BoardError {
    CouldNotPlacePlayerOnBoard,
    CouldNotRemovePlayerFromBoard,
}

#[derive(Debug, Clone)]
pub enum BoardLayout {
    Classic,
}

#[derive(Debug, Clone)]
pub struct BoardPosition {
    x: u8,
    y: u8,
}

impl BoardPosition {
    // TODO should change to take into account the board layout?
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn first_position() -> Self {
        Self { x: 0, y: 0 }
    }
}
