use crate::camera::Camera;
use crate::pixel::PixelRGB;
use crate::ray::Ray;
use crate::shapes::{HitRecord, Shape, ray_hit};
use crate::vector::Vec3;

fn gradient(ray: &Ray, color1: &PixelRGB, color2: PixelRGB) -> PixelRGB {
    let unit = ray.direction.unit_vector();
    let t = 0.7 * (unit.y + 1.0);

    ((1.0 - t) * color2) + (t * color1)
}

pub(crate) fn render(bgn_color: &PixelRGB, x: i32, y: i32, shapes: Vec<Shape>) -> Vec<PixelRGB> {
    let ns: u8 = 2;

    let capacity: usize = (x * y) as usize;
    let mut background: Vec<PixelRGB> = Vec::with_capacity(capacity + 1);

    let aspect_ratio = x as f32 / y as f32;
    let viewport_height = 2.0;
    let camera = Camera::new(Vec3::default(), viewport_height, aspect_ratio);

    for j in (0..=y - 1).rev() {
        for i in 0..x {
            let mut rec: HitRecord = Default::default();

            let mut color: Vec3 = Default::default();

            for _ in 0..ns {
                let r1: f32 = rand::random();
                let r2: f32 = rand::random();

                let u = (i as f32 + r1) / ((x as f32) - 1.0);
                let v = (j as f32 + r2) / ((y as f32) - 1.0);
                let ray = camera.ray(u, v);

                let closest = shapes
                    .iter()
                    .filter(|fig| ray_hit(&ray, *fig, &mut rec))
                    .last();

                color += if closest.is_some() {
                    Vec3::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0) * 127.5
                } else {
                    // gradient(&ray, &bgn_color, PixelRGB::white())
                    Vec3::new(255.0, 255.0, 255.0)
                };
            }

            color /= ns as f32;

            let color: PixelRGB = PixelRGB::new_vec(color);

            background.push(color);
        }
    }
    background
}
