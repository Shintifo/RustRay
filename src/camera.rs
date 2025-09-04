use crate::Vec3;
use crate::ray::Ray;

pub(crate) struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub(crate) fn new(origin: Vec3, height: f32, aspect_ratio: f32) -> Self {
        let width = height * aspect_ratio;

        let lower_left_corner: Vec3 = Vec3::new(-width / 2.0, -height / 2.0, -1.0);
        let horizontal = Vec3::new(width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, height, 0.0);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub(crate) fn ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical,
        )
    }
}
