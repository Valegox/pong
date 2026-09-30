use crate::game;
use crate::window;

const SIZE: f32 = 20.0;
const SPEED: f32 = 50.0;

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
            dir: game::Vec2(0.0, 1.0)
        }
    }

    pub fn position(&self) -> &game::Vec2 {
        &self.position
    }

    pub fn rect(&self) -> &game::Vec2 {
        &self.rect
    }

    pub fn check_collide(&self, pos: &game::Vec2, rect: &game::Vec2) {
        if is_colliding(self.position(), self.rect(), pos, rect) {
            println!("Collision detected!");
        }
    }

    pub fn apply_move(&mut self, delta_time: f32) {
        self.position.0 += self.dir.0 * self.speed * delta_time;
        self.position.1 += self.dir.1 * self.speed * delta_time;
    }
}

pub fn is_colliding(pos_a: &game::Vec2, rect_a: &game::Vec2, pos_b: &game::Vec2, rect_b: &game::Vec2) -> bool {
    let topright_a = game::Vec2(pos_a.0 + rect_a.0, pos_a.1 + rect_a.1);
    let topright_b = game::Vec2(pos_b.0 + rect_b.0, pos_b.1 + rect_b.1);
    pos_a.0 < topright_b.0 && topright_a.0 > pos_b.0 && pos_a.1 < topright_b.1 && topright_a.1 > pos_b.1
}
