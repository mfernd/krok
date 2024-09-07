use crate::game::Card;

#[derive(Debug, Clone)]
pub struct Player {
    id: PlayerIdentifier,
    username: String,
    state: PlayerState,
    color: PlayerColor,
    cards: Vec<Card>,
    remaining_bunnies: u8,
}

impl Player {
    pub fn new(id: PlayerIdentifier, username: &str, color: PlayerColor) -> Self {
        Self {
            id,
            username: username.into(),
            state: PlayerState::Alive,
            color,
            cards: Vec::new(),
            remaining_bunnies: 4,
        }
    }

    pub fn get_id(&self) -> PlayerIdentifier {
        self.id.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerIdentifier(String);

impl PlayerIdentifier {
    // TODO keep it?
    pub fn new(id: &str) -> Self {
        Self(id.into())
    }
}

#[derive(Debug, Clone)]
pub enum PlayerState {
    Alive,
    Dead,
}

#[derive(Debug, Clone)]
pub enum PlayerColor {
    Yellow,
    Red,
    Blue,
    White,
}
