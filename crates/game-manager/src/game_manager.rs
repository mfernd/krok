use crate::{
    board::BoardLayout,
    game::{Creator, Game, GameError, GameIdentifier},
    player::{Player, PlayerColor, PlayerIdentifier},
};
use rand::Rng;
use std::{collections::HashMap, iter};

pub trait GameManager {
    fn create_game(&mut self, req: CreateGameRequest) -> Result<Game, GameManagerError>;
    fn join_game(&mut self, req: JoinGameRequest) -> Result<(), GameManagerError>;
}

#[derive(Debug)]
pub struct LocalGameManager {
    games: HashMap<GameIdentifier, Game>,
}

impl GameManager for LocalGameManager {
    fn create_game(&mut self, req: CreateGameRequest) -> Result<Game, GameManagerError> {
        let creator_player_id = PlayerIdentifier::new(&req.player.id);
        let creator = Creator::from(Player::new(&req.player.username, PlayerColor::default()));

        let new_game = self
            .games
            .insert(
                self.generate_game_id()?,
                Game::init_with_creator(creator_player_id, creator, BoardLayout::Classic),
            )
            .ok_or(GameManagerError::CouldNotCreateGame(
                "Could not add game in memory".into(),
            ))?;

        Ok(new_game)
    }

    fn join_game(&mut self, req: JoinGameRequest) -> Result<(), GameManagerError> {
        let game_id = GameIdentifier::try_from(req.game_id)
            .map_err(GameManagerError::CouldNotAddPlayerToGame)?;

        self.games
            .get_mut(&game_id)
            .ok_or(GameManagerError::GameDoesNotExist)?
            .add_player(
                PlayerIdentifier::new(&req.player.id),
                Player::new(&req.player.username, PlayerColor::default()),
            )
            .map_err(GameManagerError::CouldNotAddPlayerToGame)?;

        Ok(())
    }
}

const MAX_ITERATIONS_GAME_ID_GEN: u8 = 4;
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

impl LocalGameManager {
    pub fn generate_game_id(&self) -> Result<GameIdentifier, GameManagerError> {
        let mut rng = rand::thread_rng();

        for _ in 0..MAX_ITERATIONS_GAME_ID_GEN {
            let code: String =
                iter::repeat_with(|| CHARSET[rng.gen_range(0..CHARSET.len())] as char)
                    .take(crate::game::GAME_ID_LENGTH)
                    .collect();

            let id = GameIdentifier::try_from(code).map_err(|_| {
                GameManagerError::CouldNotCreateGame("Could not generate a game_id".into())
            })?;

            if !self.games.contains_key(&id) {
                return Ok(id);
            }
        }

        Err(GameManagerError::CouldNotCreateGame(
            "Reached max game_id generation loops".into(),
        ))
    }
}

pub enum GameManagerError {
    GameDoesNotExist,
    CouldNotAddPlayerToGame(GameError),
    CouldNotCreateGame(String),
}

pub struct CreateGameRequest {
    player: GamePlayer,
}

pub struct JoinGameRequest {
    game_id: String,
    player: GamePlayer,
}

struct GamePlayer {
    id: String,
    username: String,
}
