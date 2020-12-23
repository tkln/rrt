use crate::Vec3;
use crate::Ray;
use crate::hittable::{Hittable, HitRecord};

pub struct Sphere {
    pub c: Vec3,
    pub r: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { c: center, r: radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.orig - self.c;
        let a = ray.dir.len2();
        let half_b =  oc.dot(ray.dir);
        let c = oc.len2() - self.r * self.r;
        let d = half_b * half_b -  a * c;

        if d < 0.0 {
            return None;
        }

        let sqrtd = d.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = ray.at(root);

        /* TODO Implement Vec3::Div for f32 */
        let rec = HitRecord::new(p, (p - self.c) * (1.0 / self.r), root, ray);
        return Some(rec)
    }
}
