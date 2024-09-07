use crate::{
    board::BoardLayout,
    game::{Creator, Game, GameIdentifier},
    player::{Player, PlayerColor, PlayerIdentifier},
};
use std::collections::HashMap;

pub trait GameManager {
    fn create_game(&mut self, req: CreateGameRequest);
    fn join_game(&mut self);
}

#[derive(Debug)]
pub struct SimpleGameManager {
    games: HashMap<GameIdentifier, Game>,
}

impl GameManager for SimpleGameManager {
    fn create_game(&mut self, req: CreateGameRequest) {
        let new_game = Game::new(
            GameIdentifier::new("ABCD"),
            Creator::from(Player::new(
                PlayerIdentifier::new(&req.player_id),
                &req.player_username,
                PlayerColor::Red,
            )),
            BoardLayout::Classic,
        );

        self.games.insert(new_game.get_id(), new_game);
    }

    fn join_game(&mut self) {
        unimplemented!();
    }
}

pub struct CreateGameRequest {
    player_id: String,
    player_username: String,
}
