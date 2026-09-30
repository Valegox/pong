use crate::{ball, player};

pub struct GameState {
    pub player_1: player::Player,
    pub player_2: player::Player,
    pub ball: ball::Ball
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            player_1: player::create_player_1(),
            player_2: player::create_player_2(),
            ball: ball::Ball::new()
        }
    }
}

pub struct Vec2 (pub f32, pub f32);

