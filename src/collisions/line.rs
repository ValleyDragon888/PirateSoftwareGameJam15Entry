use macroquad::color::Color;
use crate::collisions::point::Point;
use macroquad::shapes::draw_line;

pub struct Line {
    pub p1: Point,
    pub p2: Point
}

impl Line {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Line {
        Line { p1: Point {x: x1, y: y1}, p2: Point {x: x2, y: y2}  }
    }

    pub fn gradient(&self) -> f32 {
        let cx = self.p1.x - self.p2.x;
        let cy = self.p1.y - self.p2.y;
        cy / cx
    }

    pub fn draw(&self, thickness: f32, color: Color) {
        draw_line(self.p1.x, self.p1.y, self.p2.x, self.p2.y, thickness, color)
    }
}