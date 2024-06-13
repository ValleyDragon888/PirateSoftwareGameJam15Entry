use macroquad::prelude::*;

mod collisions;
use collisions::line::Line;
use collisions::point::Point;

#[macroquad::main("Desktop Build of Entry")]
async fn main() {
    let mut x = 0f32;
    let line = Line::new(0f32, 0f32, 20f32, 20f32);
    let point = Point::new(40f32, 40f32);
    loop {
        clear_background(RED);
        draw_circle(x, screen_height() - 30.0, 15.0, YELLOW);
        x += 1f32;

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        line.draw(10.0, GREEN);
        point.draw(5.0, GREEN);
        next_frame().await
    }
}