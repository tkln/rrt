use crate::Vec3;
use crate::material::Material;
use crate::hittable::{Hittable, HitRecord};
use crate::Ray;
use crate::rng::RNG;
use crate::aabb::AABB;

pub struct Tri<'a> {
    pub verts: [Vec3; 3],
    mat: &'a dyn Material,
}

impl Tri<'_> {
    pub fn new(verts: [Vec3; 3], mat: &dyn Material) -> Tri {
        Tri { verts, mat }
    }
}

impl Hittable for Tri<'_> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, _rng: &mut RNG) -> Option<HitRecord> {
        /* Möller–Trumbore intersection */
        let eps: f32 = 0.000001;
        let v0 = self.verts[0];
        let v1 = self.verts[1];
        let v2 = self.verts[2];

        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let h = ray.dir.cross(edge2);
        let a = edge1.dot(h);

        if a > -eps && a < eps {
            return None;
        }

        let f = 1.0f32 / a;
        let s = ray.orig - v0;
        let u = f * s.dot(h);

        if u < 0.0f32 || u > 1.0f32 {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * ray.dir.dot(q);

        if v < 0.0f32 || u + v > 1.0f32 {
            return None;
        }

        let t = f * edge2.dot(q);

        if t > t_min && t < t_max {
            let p = ray.at(t);
            let n = (v1 - v0).cross(v2 - v0);
            Some(HitRecord::new(p, n, t, ray, self.mat))
        } else {
            None
        }
    }

    fn get_aabb(&self) -> Option<AABB> {
        let xs: [f32; 3] = [self.verts[0].x, self.verts[1].x, self.verts[2].x];
        let ys: [f32; 3] = [self.verts[0].y, self.verts[1].y, self.verts[2].y];
        let zs: [f32; 3] = [self.verts[0].z, self.verts[1].z, self.verts[2].z];

        fn f32_ord(a: &&f32, b: &&f32) -> std::cmp::Ordering {
            if a == b {
                return std::cmp::Ordering::Equal;
            } else if a > b {
                return std::cmp::Ordering::Greater;
            } else {
                return std::cmp::Ordering::Less;
            }
        }

        Some(AABB::new(Vec3::new(*xs.iter().min_by(f32_ord).unwrap(),
                                 *ys.iter().min_by(f32_ord).unwrap(),
                                 *zs.iter().min_by(f32_ord).unwrap()),
                       Vec3::new(*xs.iter().max_by(f32_ord).unwrap(),
                                 *ys.iter().max_by(f32_ord).unwrap(),
                                 *zs.iter().max_by(f32_ord).unwrap())))
    }
}
