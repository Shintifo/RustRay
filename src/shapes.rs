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
                let a = ray.direction.squared_length();
                let b = 2.0 * oc.dot(&ray.direction);
                let c = oc.squared_length() - (radius * radius);

                let discriminant = b * b - 4.0 * a * c;
                discriminant > 0.0
            }
            Shape::Cube { center, side } => {
                let x =
                    ((center.x - side / 2.0)..=(center.x + side / 2.0)).contains(&ray.direction.x);
                let y =
                    ((center.y - side / 2.0)..=(center.y + side / 2.0)).contains(&ray.direction.y);
                let z =
                    ((center.z - side / 2.0)..=(center.z + side / 2.0)).contains(&ray.direction.z);

                x && y && z
            }
            _ => todo!(),
        }
    }
}
