use crate::ray::Ray;
use crate::vector::Vec3;
use crate::pixel::PixelRGB;

fn gradient(ray: &Ray, color: &PixelRGB) -> PixelRGB {
    let unit = ray.direction.unit_vector();
    let t = 0.5 * (unit.y + 1.0);

    (1.0 - t) * PixelRGB::white() + t * color
}

pub(crate) fn make_background(color: &PixelRGB, x: i32, y: i32) -> Vec<PixelRGB> {
    let capacity: usize = (x * y) as usize;

    if color.is_white() {
        let background: Vec<PixelRGB> = vec![Default::default(); capacity];
        background
    } else {
        let origin: Vec3 = Default::default();

        let llc: Vec3 = Vec3::new(-2.0, -1.0, -1.0);
        let width = Vec3::new(4.0, 0.0, 0.0);
        let height = Vec3::new(0.0, 4.0, 0.0);

        let mut background: Vec<PixelRGB> = Vec::with_capacity(capacity+5);

        let mut c: usize = 0;
        for j in (0..=y - 1).rev() {
            for i in 0..x {
                let u = (i as f32) / (x as f32);
                let v = (j as f32) / (y as f32);

                let ray: Ray = Ray::new(origin, llc + u * width + v * height);
                let intensity = gradient(&ray, &color) * 255.0;
                background.push(intensity);
                c += 1;
            }
        }
        background
    }
}
