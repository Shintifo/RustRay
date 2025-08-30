mod ray;
mod shapes;
mod vector;

use std::fs::File;
use std::io::Write;

use crate::ray::Ray;
use shapes::Shape;
use vector::Vec3;

fn color(ray: &Ray, base_color: &Vec3) -> Vec3 {
    let dir = ray.direction.unit_vector();
    let t: f32 = 0.5 * (dir.y + 1.0);

    let t =  (dir.y*dir.y + dir.x * dir.x).sqrt() * 2.0;

    *base_color * (2.0 - t) + Vec3::new(0.0, 0.0, 0.0) * t
    // Vec3::new(0.1, 1.0, 0.6) * t +
}

fn color_shape(ray: &Ray, shape: &Shape, base_color: &Vec3) -> Vec3 {
    if shape.ray_hit(&ray) {
        color(&ray, base_color)
    } else {
        Vec3::new(1., 1., 1.)
    }
}

fn color_shape_inv(ray: &Ray, shape: &Shape) -> Vec3 {
    if shape.ray_hit(&ray) {
        Vec3::new(1., 1., 1.)
    } else {
        Vec3::new(0., 0., 0.)
    }
}

fn show() {
    let sphere = Shape::sphere(Vec3::new(0., 0., -1.), 0.5);
    let cube = Shape::cube(Vec3::new(0., 0., -1.), 1.5);

    let cube2 = Shape::cube(Vec3::new(0., 0., -1.), 1.7);
    let cube3 = Shape::cube(Vec3::new(0., 0., -1.), 2.2);

    let mut file = File::create("gradient.ppm").unwrap();

    let nx = 200;
    let ny = 200;
    writeln!(file, "P3\n{} {}\n255", nx, ny).unwrap();

    let ll_corner = Vec3::new(-2.0, -2.0, -1.0);
    let vertical = Vec3::new(0.0, 4.0, 0.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..=ny - 1).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, ll_corner + horizontal * u + vertical * v);


            let mut color = color_shape(&r, &cube, &Vec3::new(0.1, 1.0, 0.6));
            if color == Vec3::new(1., 1., 1.) {
                color = color_shape_inv(&r, &cube2);

                if color == Vec3::new(0., 0., 0.) {
                    color = color_shape(&r, &cube3, &Vec3::new(1.0, 0.1, 0.2));
                }
            }


            let ir = (255.0 * color[0]) as u8;
            let ig = (255.0 * color[1]) as u8;
            let ib = (255.0 * color[2]) as u8;
            writeln!(file, "{} {} {}", ir, ig, ib).unwrap();
        }
    }
}

fn main() {
    show()
}
