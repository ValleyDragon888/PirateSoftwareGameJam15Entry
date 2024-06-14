use macroquad::math::Vec2;
use crate::collisions::line::Line;

struct Triangle {
    pub lines: [Line; 3],
    pub points: [Vec2; 3]
}