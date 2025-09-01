use crate::pixel::PixelRGB;
use crate::ray::Ray;
use crate::shapes::Shape;
use crate::vector::Vec3;

fn gradient(ray: &Ray, color1: &PixelRGB, color2: PixelRGB) -> PixelRGB {
    let unit = ray.direction.unit_vector();
    let t = 0.5 * (unit.y + 1.0);

    ((1.0 - t) * color2) + (t * color1)
}

pub(crate) fn render(color: &PixelRGB, x: i32, y: i32, shapes: Vec<Shape>) -> Vec<PixelRGB> {
    let capacity: usize = (x * y) as usize;

    if color.is_white() {
        let background: Vec<PixelRGB> = vec![Default::default(); capacity];
        background
    } else {
        let mut background: Vec<PixelRGB> = Vec::with_capacity(capacity + 1);
        let origin: Vec3 = Default::default();

        let aspect_ratio = x as f32 / y as f32;

        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        let llc: Vec3 = Vec3::new(-viewport_width / 2.0, -viewport_height / 2.0, -1.0);
        let width = Vec3::new(viewport_width, 0.0, 0.0);
        let height = Vec3::new(0.0, viewport_height, 0.0);

        for j in (0..=y - 1).rev() {
            for i in 0..x {
                let u = (i as f32) / ((x as f32) - 1.0);
                let v = (j as f32) / ((y as f32) - 1.0);

                let ray: Ray = Ray::new(
                    origin,
                    llc + u * width + v * height - origin,
                );

                let intensity = shapes.iter().find(|fig| fig.ray_hit(&ray)).map_or_else(
                    || gradient(&ray, &color, PixelRGB::white()) * 255.0,
                    |_| PixelRGB::new(255.0, 0.0, 0.0),
                );

                background.push(intensity);
            }
        }
        background
    }
}
