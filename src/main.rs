extern crate image;
extern crate num_cpus;
extern crate scoped_threadpool;
extern crate stackvec;

#[macro_use]
mod vector;
mod line;
mod surface;
mod shapes;

use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use obj::{load_obj, Obj};

use stackvec::prelude::*;
use scoped_threadpool::Pool;

use vector::{Point, Vector};
use line::Line;
use surface::{Triangle};
use shapes::*;

// ========================== Float & Wrapper ======================================================

const FLOAT_EPS: f64 = 1e-8;

pub fn is_zero(f: f64) -> bool {
    f.abs() <= FLOAT_EPS
}

#[derive(Copy, Clone, Debug)]
pub enum MathError {
    CollinearVectors
}

pub type MathResult<T> = Result<T, MathError>;

// ========================== Color & Environment ==================================================

type Color = [u8; 3];

struct ColoredSurface {
    triangle: Triangle,
    color: Color,
}

struct Environment {
    origin: Vector,
    sun: Vector,
    ambient_light: f32,
    diffuse_light: f32,
    grid_size: f64,
    surfaces: Vec<ColoredSurface>,
}

const IMAGE_SIZE: (u32, u32) = (500, 500);
const VOID_COLOR: [u8; 3] = [30, 30, 30];

// ========================== Ray casting ==========================================================

fn compute_lights(env: &Environment, surface: &Triangle, pt: Point) -> f32 {
    let sun_ray = Line {
        direction: vector!(pt, env.sun),
        origin: pt,
    };
    let covered = env.surfaces.iter()
        .filter(|sf| !sf.triangle.contains(pt))
        .map(|sf| sf.triangle.intersect(&sun_ray))
        // check if any intersection lies on the positive direction of the ray
        .any(|opt| opt.map(|t| t >= -FLOAT_EPS).unwrap_or(false));
    let different_halves = surface.plane.subs(env.origin)
        * surface.plane.subs(env.sun) <= 0.0;
    if covered || different_halves {
        env.ambient_light
    } else {
        let normal = surface.plane.normal();
        let cos = sun_ray.direction.cos(normal).abs() as f32;
        (1.0 - env.diffuse_light) + cos * env.diffuse_light
    }
}

fn cast_ray(env: &Environment, ray: &Line) -> [u8; 3] {
    let intersection_opt = env.surfaces.iter()
        .map(|sf: &ColoredSurface| sf.triangle.intersect(ray).map(|t| (t, sf)))
        .filter(Option::is_some).map(Option::unwrap)
        // check if it lies on the positive direction of the ray
        .filter(|is| is.0 >= -FLOAT_EPS)
        // find closest to the origin
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    if let Some((ray_param, surface)) = intersection_opt {
        let brightness = compute_lights(&env, &surface.triangle, ray.at(ray_param));
        surface.color.iter()
            .map(|c| (*c as f32 * brightness) as u8).try_collect().unwrap()
    } else {
        VOID_COLOR
    }
}

fn create_ray(env: &Environment, (x, y): (u32, u32)) -> Line {
    let interpolated = |cur: u32, max: u32| -> f64 {
        2f64 * (cur as f64 / max as f64) - 1f64
    };
    let vx = vector!(cross env.origin, vector!(axis y)).normalized();
    let vy = vector!(cross env.origin, vx).normalized();
    let pt = vector!()
        + interpolated(y, IMAGE_SIZE.1) * env.grid_size * vx
        + interpolated(x, IMAGE_SIZE.0) * env.grid_size * vy;
    Line {
        direction: vector!(env.origin, pt),
        origin: env.origin,
    }
}

fn cast_rays(env: &Environment, pool: &mut Pool) -> Vec<u8> {
    let pixel_count: usize = (IMAGE_SIZE.0 * IMAGE_SIZE.1) as usize;
    let chunks_size = pixel_count / num_cpus::get();
    let mut buff: Vec<[u8; 3]> = vec![[0, 0, 0]; pixel_count];
    pool.scoped(|scope| {
        let mut offset = 0;
        for chunk in buff.chunks_mut(chunks_size) {
            let chunk_len = chunk.len();
            scope.execute(move || {
                let rays = (0..chunk.len() as u32)
                    .map(|i| i + offset)
                    .map(|i| (i / IMAGE_SIZE.1, i % IMAGE_SIZE.1))
                    .map(|cords| create_ray(&env, cords));
                for (pixel, ray) in chunk.iter_mut().zip(rays) {
                    *pixel = cast_ray(&env, &ray);
                }
            });
            offset += chunk_len as u32;
        }
    });

    // dirty but fast cast from Vec<[u8;3]> to Vec<u8>
    unsafe {
        buff.set_len(buff.len() * 3);
        std::mem::transmute(buff)
    }
}

// ========================== Helper ===============================================================

fn parse_wavefront(filename: &str) -> Vec<ColoredSurface> {
    let mut out = vec![];
    let mut min_y = 0.0;
    let mut max_dim = 0.0;
    let y_offset = 10.0;
    let input = BufReader::new(File::open(filename).unwrap());
    let model: Obj = load_obj(input).unwrap();
    let color = [255, 100, 100];

    for tri_indices_chunk in model.indices.chunks(3) {
        let vertices: [obj::Vertex; 3] = tri_indices_chunk
            .iter().map(|idx| model.vertices[*idx as usize])
            .try_collect().unwrap();
        let points: [Point; 3] = vertices.into_iter()
            .inspect(|vert| {
                vert.position.iter()
                    .for_each(|coord| {
                        max_dim = if coord.abs() > max_dim { coord.abs() } else { max_dim }
                    })
            })
            .map(|vert| point!(vert.position[0], vert.position[1] - y_offset, vert.position[2]))
            .inspect(|vert| {
                min_y = if vert.y < min_y { vert.y } else { min_y }
            })
            .try_collect().unwrap();
        match triangle(points[0], points[1], points[2]) {
            Ok(triangle) => {
                out.push(ColoredSurface {
                    triangle,
                    color,
                })
            }
            Err(err) => {
                eprintln!("{:?}", err);
            }
        }
    }
    // push plane at minimum height
    {
        let size = 60.0;
        let (tri1, tri2) = plane(point!(0, min_y - 2.0 * FLOAT_EPS, 0), size, size)
            .unwrap();
        out.push(ColoredSurface { triangle: tri1, color: [200, 200, 200] });
        out.push(ColoredSurface { triangle: tri2, color: [200, 200, 200] });
    }
    out
}

fn main() {
    let mut env = Environment {
        origin: vector!(-5, 70, 0),
        sun: vector!(-80, 150, 80),
        ambient_light: 0.4,
        diffuse_light: 0.2,
        grid_size: 40.0,
        surfaces: parse_wavefront("test/tower.obj"),
    };
    let mut thread_pool = Pool::new(num_cpus::get() as u32);
    let origin_radius = 160f64;
    let steps: usize = 65;

    for step in 0..20 {
        let percent = step as f64 / steps as f64;
        let angle: f64 = percent * 2.0 * std::f64::consts::PI;
        env.origin.x = angle.sin() * origin_radius;
        env.origin.z = angle.cos() * origin_radius;

        let buffer = cast_rays(&env, &mut thread_pool);
        image::save_buffer(&Path::new(&format!("test/output{}.png", step)),
                           &buffer, IMAGE_SIZE.0, IMAGE_SIZE.1, image::ColorType::Rgb8)
            .expect("failed to write image");
    }
}
