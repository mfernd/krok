use crate::game::Card;

#[derive(Debug, Clone)]
pub struct Player {
    username: String,
    state: PlayerState,
    color: PlayerColor,
    cards: Vec<Card>,
    remaining_bunnies: u8,
}

impl Player {
    pub fn new(username: &str, color: PlayerColor) -> Self {
        Self {
            username: username.into(),
            state: PlayerState::Alive,
            color,
            cards: Vec::new(),
            remaining_bunnies: 4,
        }
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

impl Default for PlayerColor {
    fn default() -> Self {
        Self::Red
    }
}
