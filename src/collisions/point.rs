use macroquad::color::{Color, YELLOW};
use macroquad::prelude::draw_circle;

pub struct Point {
    pub x:f32,
    pub y:f32
}

impl Point {
    pub fn new(x: f32, y:f32) -> Point {
        Point {x, y}
    }

    pub fn draw(&self, radius: f32, color: Color) {
        draw_circle(self.x, self.y, radius, color);
    }
}