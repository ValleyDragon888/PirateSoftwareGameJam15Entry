use macroquad::prelude::*;

mod collisions;
use collisions::line::Line;

#[macroquad::main("Desktop Build of Entry")]
async fn main() {
    let mut x = 0f32;
    let line = Line::new(0f32, 0f32, 20f32, 20f32);
    loop {
        clear_background(RED);
        draw_circle(x, screen_height() - 30.0, 15.0, YELLOW);
        x += 1f32;

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        line.draw(10.0, GREEN);
        next_frame().await
    }
}