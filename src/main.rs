mod background;
mod camera;
mod pixel;
mod ray;
mod shapes;
mod util;
mod vector;

use crate::background::render;
use crate::pixel::PixelRGB;
use shapes::Shape;
use std::fs::File;
use std::io::Write;
use vector::Vec3;

fn plot() {
    let nx = 1000;
    let ny = 500;

    let color = PixelRGB::new(127, 127, 255);

    let mut shapes: Vec<Shape> = Vec::new();
    shapes.push(Shape::sphere(Vec3::new(0.0, 0.0, -1.0), 0.5));
    shapes.push(Shape::sphere(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let pixels = render(&color, nx, ny, shapes);

    let mut file = File::create("test.ppm").unwrap();
    writeln!(file, "P3\n{} {}\n255", nx, ny).unwrap();

    for pixel in pixels {
        writeln!(file, "{}", pixel).unwrap();
    }
}

fn main() {
    plot();
}
