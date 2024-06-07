use macroquad::prelude::*;

#[macroquad::main("Desktop Build of Entry")]
async fn main() {
    let mut x = 0f32;
    loop {
        clear_background(RED);

        //draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        //draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(x, screen_height() - 30.0, 15.0, YELLOW);
        x += 1f32;

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}