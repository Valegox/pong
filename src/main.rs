use std::time::Instant;
use minifb::{Key, KeyRepeat, Window, WindowOptions};

mod window;
mod game;
mod player;
mod input;
mod ball;

fn main() {
    let mut window = Window::new(
        "Pong",
        window::WIDTH,
        window::HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    let mut game_state= game::GameState::new();
    let mut input_state = input::InputState::new(vec![Key::A, Key::S, Key::K, Key::L]);
    
    let mut last_frame = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // Calculate delta time
        let now = Instant::now();
        let delta_time = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;

        // Update inputs
        let keys_pressed = window.get_keys_pressed(KeyRepeat::No);
        let keys_released = window.get_keys_released();
        input_state.update(keys_pressed, keys_released);
        
        // Move players
        let player_1_dir: i8 = input_state.get_dir(&Key::A, &Key::S);
        let player_2_dir: i8 = input_state.get_dir(&Key::K, &Key::L);
        game_state.player_1.apply_move(game::Vec2(player_1_dir as f32, 0.0), delta_time);
        game_state.player_2.apply_move(game::Vec2(player_2_dir as f32, 0.0), delta_time);

        // Move ball
        game_state.ball.check_collide(game_state.player_1.position(), game_state.player_1.rect());
        game_state.ball.check_collide(game_state.player_2.position(), game_state.player_2.rect());
        game_state.ball.apply_move(delta_time);

        // Draw entities
        let mut buffer: Vec<u32> = vec![0; window::WIDTH * window::HEIGHT];
        window::draw_rect(&mut buffer, game_state.player_1.position(), game_state.player_1.rect());
        window::draw_rect(&mut buffer, game_state.player_2.position(), game_state.player_2.rect());
        window::draw_rect(&mut buffer, game_state.ball.position(), game_state.ball.rect());
        
        window
            .update_with_buffer(&buffer, window::WIDTH, window::HEIGHT)
            .unwrap();
    }
}
