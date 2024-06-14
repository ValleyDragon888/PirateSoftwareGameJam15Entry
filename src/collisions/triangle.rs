use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::shapes::draw_triangle;
use crate::collisions::line::Line;

struct Triangle {
    pub lines: [Line; 3],
    pub points: [Vec2; 3]
}

impl Triangle {
    pub fn draw(&self, color: Color) {
        draw_triangle(self.points[0], self.points[1], self.points[2], color)
    }
}