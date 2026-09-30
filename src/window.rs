use crate::game;

pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 480;

pub fn draw_rect(buffer: &mut Vec<u32>, position: &game::Vec2, rect: &game::Vec2) {
    for x in position.0 as i32..(rect.0 + position.0) as i32 {
        if x < 0 || x as usize >= WIDTH {
            continue;
        }
        for y in position.1 as i32..(rect.1 + position.1) as i32 {
            if y < 0 || y as usize >= HEIGHT {
                continue;
            }
            let i = y as usize * WIDTH + x as usize;
            buffer[i] = 0x00FFFFFF;//0x000000FF;
        }
    }
}
