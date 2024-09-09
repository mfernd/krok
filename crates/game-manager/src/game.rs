use crate::{
    board::{Board, BoardError, BoardLayout},
    player::{self, Player, PlayerIdentifier},
};
use std::collections::HashMap;

pub const GAME_ID_LENGTH: usize = 6;

#[derive(Debug, Clone)]
pub struct Game {
    state: GameState,
    creator: Creator,
    /// also include the game's creator
    participants: HashMap<PlayerIdentifier, Player>,
    board: Board,
    deck: Deck,
}

impl Game {
    pub fn init_with_creator(
        creator_id: PlayerIdentifier,
        creator: Creator,
        layout: BoardLayout,
    ) -> Self {
        let creator_player = creator.0.clone();

        Self {
            state: GameState::WaitingForPlayer,
            creator,
            participants: HashMap::from([(creator_id.clone(), creator_player)]),
            board: Board::new_with_first_player(layout, creator_id),
            deck: Deck { cards: Vec::new() },
        }
    }

    pub fn add_player(
        &mut self,
        player_id: PlayerIdentifier,
        new_player: Player,
    ) -> Result<(), GameError> {
        if self.participants.len() == 4 {
            return Err(GameError::GameIsFull);
        }

        self.board
            .add_player_at_the_start(player_id.clone())
            .map_err(GameError::Board)?;

        self.participants
            .insert(player_id.clone(), new_player)
            .ok_or_else(|| {
                let _ = self.board.remove_player(&player_id);

                GameError::CouldNotAddPlayerInParticipants
            })?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum GameError {
    GameIsFull,
    CouldNotAddPlayerInParticipants,
    Board(BoardError),
    InvalidGameId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameIdentifier(String);

impl TryFrom<String> for GameIdentifier {
    type Error = GameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if GAME_ID_LENGTH < value.len() {
            return Err(GameError::InvalidGameId);
        }

        Ok(Self(value))
    }
}

#[derive(Debug, Clone)]
pub enum GameState {
    WaitingForPlayer,
    WaitingToStart,
    /// Also contains which player has to play.
    Playing(Player),
    Ended,
}

#[derive(Debug, Clone)]
pub struct Creator(Player);

impl From<Player> for Creator {
    fn from(value: Player) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

#[derive(Debug, Clone)]
pub enum Card {
    MoveNStepForward(u8),
    RotateBoard,
}
