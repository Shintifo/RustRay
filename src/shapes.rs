use crate::ray::Ray;
use crate::vector::Vec3;

pub(crate) enum Shape {
    Sphere {
        center: Vec3,
        radius: f32,
    },
    Cube {
        center: Vec3,
        side: f32,
    },
    Cylinder {
        center: Vec3,
        radius: f32,
        height: f32,
    },
}

impl Shape {
    pub(crate) fn sphere(center: Vec3, radius: f32) -> Shape {
        Shape::Sphere { center, radius }
    }

    pub(crate) fn cube(center: Vec3, side: f32) -> Shape {
        Shape::Cube { center, side }
    }

    pub(crate) fn ray_hit(&self, ray: &Ray) -> bool {
        match self {
            Shape::Sphere { center, radius } => {
                let oc = ray.origin - *center;
                let a = ray.direction.dot(&ray.direction);
                let b = 2.0 * ray.direction.dot(&oc);
                let c = oc.dot(&oc) - (radius * radius);

                let discriminant = b * b - 4.0 * a * c;
                discriminant > 0.0
            }
            Shape::Cube { center, side } => {
                let half = side / 2.0;
                let x = ((center.x - half)..=(center.x + half)).contains(&ray.direction.x);
                let y = ((center.y - half)..=(center.y + half)).contains(&ray.direction.y);
                let z = ((center.z - half)..=(center.z + half)).contains(&ray.direction.z);

                x && y && z
            }
            _ => todo!(),
        }
    }
}
