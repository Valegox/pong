use crate::{game, window};

const WIDTH: f32 = 20.0;
const HEIGHT: f32 = 100.0;
const MARGIN: f32 = 20.0;
const SPEED: f32 = 300.0;

pub struct Player {
    position: game::Vec2,
    rect: game::Vec2,
    speed: f32
}

impl Player {
    fn new(position: game::Vec2) -> Player {
        Player {
            position,
            rect: game::Vec2(WIDTH, HEIGHT),
            speed: SPEED
        }
    }

    pub fn position(&self) -> &game::Vec2 {
        &self.position
    }

    pub fn rect(&self) -> &game::Vec2 {
        &self.rect
    }

    pub fn apply_move(&mut self, dir: game::Vec2, delta_time: f32) {
        self.position.0 += dir.0 * self.speed * delta_time;
        self.position.1 += dir.1 * self.speed * delta_time;

        if self.position.1 < 0.0 {
            self.position.1 = 0.0;
        } else if self.position.1 > window::HEIGHT as f32 - self.rect.1 {
            self.position.1 = window::HEIGHT as f32 - self.rect.1;
        }
    }

}

pub fn create_player_1() -> Player {
    Player::new(game::Vec2(
        MARGIN,
        window::HEIGHT as f32 / 2.0 - HEIGHT / 2.0
    ))
}

pub fn create_player_2() -> Player {
    Player::new(game::Vec2(
        window::WIDTH as f32 - MARGIN * 2.0,
        window::HEIGHT as f32 / 2.0 - HEIGHT / 2.0
    ))
}
