use crate::game;
use crate::window;

const SIZE: f32 = 20.0;
const SPEED: f32 = 200.0;

pub struct Ball {
    position: game::Vec2,
    rect: game::Vec2,
    speed: f32,
    dir: game::Vec2
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            position: game::Vec2(window::WIDTH as f32 / 2.0, window::HEIGHT as f32 / 2.0),
            rect: game::Vec2(SIZE, SIZE),
            speed: SPEED,
            dir: rand_dir()
        }
    }

    pub fn position(&self) -> &game::Vec2 {
        &self.position
    }

    pub fn rect(&self) -> &game::Vec2 {
        &self.rect
    }

    pub fn handle_player_collision(&mut self, pos: &game::Vec2, rect: &game::Vec2, bounce_dir: i8) {
        if is_colliding(self.position(), self.rect(), pos, rect) {
            self.dir = game::Vec2(bounce_dir as f32 * self.dir.0.abs(), self.dir.1);
        }
    }

    pub fn handle_border_collision(&mut self) {
        if self.position.1 <= 0.0 {
            self.dir = game::Vec2(self.dir.0, self.dir.1.abs());
        }
        if self.position.1 >= window::HEIGHT as f32 - self.rect.1 {
            self.dir = game::Vec2(self.dir.0, -self.dir.1.abs());
        }
    }

    pub fn apply_move(&mut self, delta_time: f32) {
        self.position.0 += self.dir.0 * self.speed * delta_time;
        self.position.1 += self.dir.1 * self.speed * delta_time;
    }

    pub fn check_victory(&self, score: &mut (u32, u32)) -> bool {
        if self.position.0 <= 0.0 {
            score.1 += 1;
            return true;
        }
        if self.position.0 >= window::WIDTH as f32 - self.rect.0 {
            score.0 += 1;
            return true;
        }
        false
    }
}

fn is_colliding(pos_a: &game::Vec2, rect_a: &game::Vec2, pos_b: &game::Vec2, rect_b: &game::Vec2) -> bool {
    let topright_a = game::Vec2(pos_a.0 + rect_a.0, pos_a.1 + rect_a.1);
    let topright_b = game::Vec2(pos_b.0 + rect_b.0, pos_b.1 + rect_b.1);
    pos_a.0 < topright_b.0 && topright_a.0 > pos_b.0 && pos_a.1 < topright_b.1 && topright_a.1 > pos_b.1
}

fn rand_dir() -> game::Vec2 {
    let x= rand_dir_value();
    let y = rand_dir_value();
    game::Vec2(x, y).normalized()
}

fn rand_dir_value() -> f32 {
    let is_neg = rand::random_bool(0.5);
    let value = rand::random_range(0.2..=1.0);
    match is_neg {
        true => -value,
        false => value
    }
}
