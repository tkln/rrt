use crate::Vec3;
use crate::Ray;

pub struct HitRecord {
    pub p: Vec3,
    pub n: Vec3,
    pub t: f32,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
