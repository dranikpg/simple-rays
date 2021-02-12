use super::*;
use std::ops::{Add, Mul, Index, IndexMut};

// Point & Vector
#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Vector = Point;

macro_rules! point {
    ($x:expr, $y: expr, $z: expr) =>
        {crate::vector::Point{x: $x as f64, y:  $y as f64, z: $z as f64}}
}

macro_rules! vector {
    (axis x) => {vector!(1, 0, 0)};
    (axis y) => {vector!(0, 1, 0)};
    (axis z) => {vector!(0, 0, 1)};
    () => {vector!(0,0,0)};
    ($a: expr, $b: expr) => {vector!($b.x-$a.x, $b.y-$a.y, $b.z-$a.z)};
    ($x:expr, $y: expr, $z: expr) => {point!($x, $y, $z) as crate::vector::Vector};
    (cross $a:expr, $b: expr) => {$a.cross($b)};
}

impl Vector {
    pub fn len(&self) -> f64 {
        let Vector { x, y, z } = self;
        (x * x + y * y + z * z).sqrt()
    }
    pub fn normalized(&self) -> Self {
        if !self.is_zero() {
            let len = self.len();
            vector!(self.x / len, self.y / len, self.z / len)
        } else {
            vector!()
        }
    }
    pub fn dot(&self, v2: Vector) -> f64 {
        let Vector { x, y, z } = self;
        x * v2.x + y * v2.y + z * v2.z
    }
    pub fn cos(&self, v2: Vector) -> f64 {
        self.dot(v2) / (self.len() * v2.len())
    }
    pub fn cross(&self, v2: Vector) -> Vector {
        Vector {
            x:  (self.y * v2.z - self.z * v2.y),
            y: -(self.x * v2.z - self.z * v2.x),
            z:  (self.x * v2.y - self.y * v2.x),
        }
    }
    pub fn is_zero(&self) -> bool {
        return is_zero(self.x) && is_zero(self.y) && is_zero(self.z);
    }
    pub fn is_collinear(&self, v2: Vector) -> bool {
        self.cross(v2).is_zero()
    }
    pub fn is_codirectional(&self, v2: Vector) -> bool {
        return self.is_collinear(v2) && self.x * v2.x >= -FLOAT_EPS
            && self.y * v2.y >= -FLOAT_EPS && self.z * v2.z >= -FLOAT_EPS;
    }
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector { x: rhs * self.x, y: rhs * self.y, z: rhs * self.z }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

impl Index<usize> for Vector {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable!()
        }
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => unreachable!()
        }
    }
}