use crate::Vec3;
use crate::Ray;

pub struct HitRecord {
    pub p: Vec3,
    pub n: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vec3, out_normal: Vec3, t: f32, ray: &Ray) -> HitRecord {
        let front_face = ray.dir.dot(out_normal) < 0.0;
        HitRecord {
            p: p,
            n: if front_face { out_normal } else { -out_normal },
            t: t,
            front_face: front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
        let mut res = iter.filter_map(|item| item.hit(ray, t_min, t_max)).peekable();
        if res.peek().is_none() {
            return None;
        }
        let min = res.min_by(hit_ord).unwrap();
        return Some(min);
    }
}
