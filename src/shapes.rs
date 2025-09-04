use crate::ray::Ray;
use crate::vector::Vec3;
use std::cmp::min;

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

pub(crate) struct HitRecord {
    pub(crate) t: f32,
    pub(crate) p: Vec3,
    pub(crate) normal: Vec3,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            t: f32::MAX,
            p: Vec3::default(),
            normal: Vec3::default(),
        }
    }
}

pub(crate) trait RayHit {
    fn ray_hit(&self, ray: &Ray, rec: &mut HitRecord) -> bool;
}

impl RayHit for Shape {
    fn ray_hit(&self, ray: &Ray, rec: &mut HitRecord) -> bool {
        let unit = ray.direction.unit_vector();
        let threshold: f32 = 1e-6;
        match self {
            Shape::Sphere { center, radius } => {
                let oc = ray.origin - *center;
                let a = unit.dot(&unit);
                let half_b = unit.dot(&oc);
                let c = oc.dot(&oc) - (radius * radius);

                let discriminant = half_b * half_b - a * c;
                if discriminant < 0.0 {
                    return false;
                }

                let sqrtd = discriminant.sqrt();
                let t1 = (-half_b - sqrtd) / a;
                let t2 = (-half_b + sqrtd) / a;

                if (t1 < threshold || t1 > rec.t) && (t2 < threshold || t2 > rec.t) {
                    return false;
                }

                rec.t = t1.min(t2);
                rec.p = ray.at(rec.t);
                rec.normal = (rec.p - *center).unit_vector();

                true
            }
            Shape::Cube { center, side } => {
                let half = side / 2.0;
                let x = ((center.x - half)..=(center.x + half)).contains(&ray.direction.x);
                let y = ((center.y - half)..=(center.y + half)).contains(&ray.direction.y);
                let z = ((center.z - half)..=(center.z + half)).contains(&ray.direction.z);

                //TODO: correctly define the t
                x && y && z
            }
            _ => todo!(),
        }
    }
}

pub(crate) fn ray_hit<T: RayHit>(ray: &Ray, obj: &T, rec: &mut HitRecord) -> bool {
    obj.ray_hit(ray, rec)
}
