use crate::{
    board::{Board, BoardLayout},
    player::Player,
};

#[derive(Debug)]
pub struct Game {
    id: GameIdentifier,
    state: GameState,
    creator: Creator,
    /// also include the game's creator
    participants: Vec<Player>,
    board: Board,
    deck: Deck,
}

impl Game {
    pub fn new(id: GameIdentifier, creator: Creator, layout: BoardLayout) -> Self {
        let creator_player = creator.0.clone();
        let creator_id = creator.0.get_id();

        Self {
            id,
            state: GameState::WaitingForPlayer,
            creator,
            participants: Vec::from([creator_player]),
            board: Board::new_with_first_player(layout, creator_id),
            deck: Deck { cards: Vec::new() },
        }
    }

    pub fn get_id(&self) -> GameIdentifier {
        self.id.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameIdentifier(String);

impl GameIdentifier {
    // TODO keep it?
    pub fn new(id: &str) -> Self {
        Self(id.into())
    }
}

#[derive(Debug)]
pub enum GameState {
    WaitingForPlayer,
    CanStart,
    /// Also contains which player has to play.
    Playing(Player),
    Ended,
}

#[derive(Debug)]
pub struct Creator(Player);

impl From<Player> for Creator {
    fn from(value: Player) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

#[derive(Debug, Clone)]
pub enum Card {
    MoveNStepForward(u8),
    RotateBoard,
}
