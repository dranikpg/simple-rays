use super::surface::{Triangle};
use super::vector::Point;
use crate::MathResult;

pub fn triangle(p1: Point, p2: Point, p3: Point) -> MathResult<Triangle> {
    Triangle::new(p1, p2, p3)
}

pub fn quad(p1: Point, p2: Point, p3: Point, p4: Point) -> MathResult<(Triangle, Triangle)> {
    Ok((
        triangle(p1, p2, p3)?,
        triangle(p1, p2, p4)?
    ))
}

pub fn plane(center: Point, length: f32, width: f32) -> MathResult<(Triangle, Triangle)> {
    let p1 = center + point!(length / 2f32, 0, width / 2f32);
    let p2 = center + point!(-length / 2f32, 0, -width / 2f32);
    quad(p1, p2, center + point!(length / 2f32, 0, - width / 2f32),
         center + point!(- length / 2f32, 0, width / 2f32))
}

pub fn tetrahedron(p1: Point, p2: Point, p3: Point, h: Point) -> MathResult<Vec<Triangle>> {
    Ok(vec![
        triangle(p1, p2, h)?,
        triangle(p2, p3, h)?,
        triangle(p3, p1, h)?,
        triangle(p1, p2, p3)?
    ])
}

pub fn cube(center: Point, size: f64) -> MathResult<Vec<Triangle>> {
    let mut out = vec![];
    for dim in 0..3 {
        for side in &[-1, 1] {
            let mut vs = vec![];
            for pt in 0..4 {
                let mut diff = vector!();
                diff[dim] = *side as f64 * size / 2.0;
                let (d1, d2) = {
                    let mut idx_id = (0..3).filter(|i| *i != dim);
                    (idx_id.next().unwrap(), idx_id.next().unwrap())
                };
                let (ds1, ds2) = match pt {
                    0 => (-size / 2.0, -size / 2.0),
                    1 => (size / 2.0, size / 2.0),
                    2 => (size / 2.0, -size / 2.0),
                    3 => (-size / 2.0, size / 2.0),
                    _ => unreachable!()
                };
                diff[d1] = ds1;
                diff[d2] = ds2;
                vs.push(center + diff);
            }
            let qd = quad(vs[0], vs[1], vs[2], vs[3])?;
            out.push(qd.0);
            out.push(qd.1);
        }
    }
    Ok(out)
}

