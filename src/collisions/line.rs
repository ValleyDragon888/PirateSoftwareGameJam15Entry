use crate::collisions::point::Point;

pub struct Line {
    p1: Point,
    p2: Point
}

impl Line {
    pub fn gradient(&self) -> f32 {
        let cx = self.p1.x - self.p2.x;
        let cy = self.p1.y - self.p2.y;
        cy / cx
    }
}