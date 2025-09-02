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
}

pub(crate) trait HitAble {
    fn ray_hit(&self, ray: &Ray) -> f32;
}

impl HitAble for Shape {
    fn ray_hit(&self, ray: &Ray) -> f32 {
        let unit = ray.direction.unit_vector();

        match self {
            Shape::Sphere { center, radius } => {
                let oc = ray.origin - *center;
                let a = unit.dot(&unit);
                let half_b = unit.dot(&oc);
                let c = oc.dot(&oc) - (radius * radius);

                let discriminant = half_b * half_b - a * c;
                if discriminant < 0.0 {
                    return -1.0;
                }

                let sqrtd = discriminant.sqrt();
                let t1 = (-half_b - sqrtd) / a;
                let t2 = (-half_b + sqrtd) / a;

                // TODO: what root we should use?
                if t1 > 1e-6 {
                    return t1;
                }
                if t2 > 1e-6 {
                    return t2;
                }
                -1.0
            }
            Shape::Cube { center, side } => {
                let half = side / 2.0;
                let x = ((center.x - half)..=(center.x + half)).contains(&ray.direction.x);
                let y = ((center.y - half)..=(center.y + half)).contains(&ray.direction.y);
                let z = ((center.z - half)..=(center.z + half)).contains(&ray.direction.z);

                if x && y && z { 1.0 } else { -1.0 }
            }
            _ => todo!(),
        }
    }
}

pub(crate) fn ray_hit<T: HitAble>(ray: &Ray, obj: &T) -> f32 {
    obj.ray_hit(ray)
}
