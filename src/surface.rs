use super::*;
use super::vector::*;
use super::line::Line;

#[derive(Debug)]
pub struct Plane {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl Plane {
    pub fn new(p: Point, v1: Vector, v2: Vector) -> MathResult<Self> {
        match v1.cross(v2) {
            v if v.is_zero() => Err(MathError::CollinearVectors),
            Vector { x, y, z } => Ok(Plane {
                a: x,
                b: y,
                c: z,
                d: -(x * p.x + y * p.y + z * p.z),
            })
        }
    }
    pub fn intersect(&self, line: &Line) -> Option<f64> {
        let Line { direction, origin } = line;
        let sum_t = self.a * direction.x + self.b * direction.y + self.c * direction.z;
        let sum_rhs = -self.d - self.a * origin.x - self.b * origin.y - self.c * origin.z;
        if is_zero(sum_t) {
            None
        } else {
            Some(sum_rhs / sum_t)
        }
    }
    pub fn contains(&self, pt: Point) -> bool {
        is_zero(self.subs(pt))
    }
    pub fn subs(&self, pt: Point) -> f64 {
        self.a * pt.x + self.b * pt.y + self.c * pt.z + self.d
    }
    pub fn normal(&self) -> Vector {
        vector!(self.a, self.b, self.c)
    }
}

#[derive(Debug)]
pub struct Triangle {
    pub vertices: [Point; 3],
    pub plane: Plane,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> MathResult<Self> {
        let plane = Plane::new(p1, vector!(p1, p2), vector!(p1, p3))?;
        Ok(Triangle {
            vertices: [p1, p2, p3],
            plane,
        })
    }
    pub fn intersect(&self, line: &Line) -> Option<f64> {
        let (intersection, param) = match self.plane.intersect(line) {
            Some(t) => (line.at(t), t),
            None => return None,
        };
        if self.is_inside(intersection) {
            Some(param)
        } else {
            None
        }
    }
    pub fn contains(&self, pt: Point) -> bool {
        self.plane.contains(pt) && self.is_inside(pt)
    }
    fn is_inside(&self, pt: Point) -> bool {
        self.vertices.iter().enumerate()
            // for each vertex calculate the cross product of
            // 1) the segment between the vertex and the intersection point
            // 2) the next triangle side
            .map(|(pos, vertex)| -> Vector {
                let next_vertex = self.vertices[(pos + 1) % 3];
                vector!(cross   vector!(vertex, pt),
                                vector!(vertex, next_vertex))
            })
            // check if each vector `v` is codirectional with the sum of the previous ones
            // that is equivalent to all of them being pairwise codirectional
            .fold(Some(vector!()), |last_opt, v| -> Option<Vector> {
                match last_opt {
                    Some(last) => if v.is_codirectional(last) { Some(v + last) } else { None }
                    None => None,
                }
            }).is_some()
    }
}