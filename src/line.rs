use crate::vector::*;

#[derive(Debug)]
pub struct Line {
    pub direction: Vector,
    pub origin: Point,
}

impl Line {
    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }
}
