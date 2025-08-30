mod ray;
mod vector;

use std::fs::File;
use std::io::Write;

use crate::ray::Ray;
use vector::Vec3;

fn color(ray: &Ray) -> Vec3 {
    let dir = ray.direction.unit_vector();
    let t: f32 = 0.5 * (dir.y+1.0);

    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.1, 0.7, 1.0) * t
}

fn show() {
    let mut file = File::create("gradient.ppm").unwrap();

    let nx = 200;
    let ny = 100;
    writeln!(file, "P3\n{} {}\n255", nx, ny).unwrap();

    let ll_corner = Vec3::new(-2.0, -1.0, -1.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..=ny - 1).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, ll_corner + horizontal * u + vertical * v);

            let color = color(&r);

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
