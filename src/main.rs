mod sprite;

use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;
use sprite::Player;
use crate::maths::vec2s;

mod maths;


#[macroquad::main("Desktop Build of Entry")]
async fn main() {
    let texture: Texture2D = load_texture("assets/thor-highquality.png").await.unwrap();
    let mut player = Player {
        pos: vec2s(0.1, 0.1),
        texture2d: texture
    };
    loop {
        // // Set camera
        // let view_size = vec2(720.0f32, 240.0f32);
        // let window_size = vec2(screen_size().0, screen_size().1);
        //
        // let scale = (window_size / view_size).min_element().floor().max(1.0);
        // let remaining_size = (window_size / scale) - view_size;
        // let camera_size = view_size + remaining_size;
        //
        // set_camera(&Camera2D::from_display_rect(Rect::new( 0.0, camera_size.y, camera_size.x, -camera_size.y)));


        player.update();
        draw_circle(0.0, 0.0, 200.0, RED);
        next_frame().await;
    }
}