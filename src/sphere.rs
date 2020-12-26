use crate::Vec3;
use crate::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::material::Material;
use crate::rng::RNG;

pub struct Sphere<'a> {
    pub c: Vec3,
    pub r: f32,
    mat: &'a dyn Material,
}

impl Sphere<'_> {
    pub fn new(center: Vec3, radius: f32, mat: &dyn Material) -> Sphere {
        Sphere { c: center, r: radius, mat: mat }
    }
}

impl Hittable for Sphere<'_> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, _rng: &mut RNG) -> Option<HitRecord> {
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

        Some(HitRecord::new(p, (p - self.c) / self.r, root, ray, self.mat))
    }
}
