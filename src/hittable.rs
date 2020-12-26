use crate::Vec3;
use crate::Ray;
use crate::material::Material;
use crate::rng::RNG;

pub struct HitRecord<'a> {
    pub p: Vec3,
    pub n: Vec3,
    pub mat: &'a dyn Material,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord<'_> {
    pub fn new<'a>(p: Vec3, out_normal: Vec3, t: f32, ray: &Ray,
                   material: &'a dyn Material) -> HitRecord<'a> {
        let front_face = ray.dir.dot(out_normal) < 0.0;
        HitRecord {
            p: p,
            n: if front_face { out_normal } else { -out_normal },
            t: t,
            mat: material,
            front_face: front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rng: &mut RNG) -> Option<HitRecord>;
}

pub struct HittableList<'a> {
    pub hittables: Vec<&'a dyn Hittable>,
}

impl Hittable for HittableList<'_> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rng: &mut RNG) -> Option<HitRecord> {
        fn hit_ord(hit_a: &HitRecord, hit_b: &HitRecord) -> std::cmp::Ordering {
            if hit_a.t == hit_b.t {
                return std::cmp::Ordering::Equal;
            } else if hit_a.t > hit_b.t {
                return std::cmp::Ordering::Greater;
            } else {
                return std::cmp::Ordering::Less;
            }
        }

        let iter = (&self.hittables).into_iter();
        /* Call Hittable::hit on each an filter out Nones */
        let mut res = iter.filter_map(|item| item.hit(ray, t_min, t_max, rng)).peekable();
        if res.peek().is_none() {
            return None;
        }
        let min = res.min_by(hit_ord).unwrap();
        return Some(min);
    }
}
