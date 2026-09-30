use crate::{ball, player};

pub struct GameState {
    pub player_1: player::Player,
    pub player_2: player::Player,
    pub ball: ball::Ball,
    pub score: (u32, u32)
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            player_1: player::create_player_1(),
            player_2: player::create_player_2(),
            ball: ball::Ball::new(),
            score: (0, 0)
        }
    }
    
    pub fn reset(&mut self) {
        self.player_1 = player::create_player_1();
        self.player_2 =  player::create_player_2();
        self.ball = ball::Ball::new();
    }
}

pub struct Vec2 (pub f32, pub f32);

impl Vec2 {
    pub fn length(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }

    pub fn normalized(&self) -> Vec2 {
        let len = self.length();
        Vec2(self.0 / len, self.1 / len)
    }
}

