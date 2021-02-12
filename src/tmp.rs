fn tmp1() {
    let tri1 = Surface::new(point!(10,-3,-1), point!(0,-3,-5), point!(0,-3,10))
        .unwrap();
    let tri2 = Surface::new(point!(4,2,0), point!(1,-2,0), point!(2,-2,3))
        .unwrap();
    let tri3 = Surface::new(point!(20,-4,-10), point!(20,-4,10), point!(-2,-4,-1))
        .unwrap();
}

fn tmp2() -> Vec<ColoredSurface> {
    let mut env = Environment {
        origin: vector!(-5, 3, 1.25),
        sun: vector!(0, 5, 0),
        ambient_light: 0.4,
        phong_shading: 0.2,
        surfaces: build_shapes(),
    };
    let mut shapes = vec![];
    {
        let (tri1, tri2) = plane(point!(0.3,0,0), 5f32, 5f32);
        shapes.push(ColoredSurface { surface: tri1, color: (0, 255, 0) });
        shapes.push(ColoredSurface { surface: tri2, color: (0, 255, 0) });
    }
    {
        let vs = tetrahedron(point!(-1,0,0.25), point!(1,0,0.25),
                             point!(-1,0,2.25), point!(0,1,1.25));
        shapes.extend(vs.into_iter().map(|sf|
            ColoredSurface { surface: sf, color: (0, 0, 255) }
        ))
    }
    {
        let vs = cube(point!(0.25,0,-0.8), 1.0);
        shapes.extend(vs.into_iter().map(|sf|
            ColoredSurface { surface: sf, color: (255, 0, 0) }
        ))
    }
    shapes
}

fn tmp3() {
    let mut shapes = vec![];
    {
        let (tri1, tri2) = plane(point!(0,0,0), 5f32, 5f32);
        shapes.push(ColoredSurface { triangle: tri1, color: [0, 255, 0] });
        shapes.push(ColoredSurface { triangle: tri2, color: [0, 255, 0] });
    }
    for step in 0..10 {
        let radius = 2.0;
        let percent = step as f32 / 10.0;
        let angle = percent * 2.0 * std::f32::consts::PI;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        let x2 = (angle + 0.3).cos() * radius;
        let z2 = (angle + 0.3).sin() * radius;
        let surface = triangle(point!(0,2,0), point!(x,0,z), point!(x2,0,z2));
        let color = [(percent * 255.0) as u8, 0, 255];
        shapes.push(ColoredSurface { triangle: surface, color });
    }
}