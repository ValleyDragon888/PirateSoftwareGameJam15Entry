use crate::collisions::line::Line;
use crate::collisions::point::Point;

struct Triangle {
    pub lines: [Line; 3],
    pub points: [Point; 3]
}