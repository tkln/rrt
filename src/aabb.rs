use crate::Vec3;
use crate::Ray;
use crate::hittable::Hittable;

#[derive(Copy, Clone)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        assert!(min.x <= max.x && min.y <= max.y && min.z <= max.z);
        AABB { min, max }
    }

    /* This should only be used as a initial value for union */
    pub fn zero() -> AABB {
        AABB { min: Vec3::zero(), max: Vec3::zero() }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let invd = ray.dir.recip();
        let mut t0 = (self.min - ray.orig) * invd;
        let mut t1 = (self.max - ray.orig) * invd;

        let mut t0_min = t_min;
        let mut t1_max = t_max;

        for a in 0..3 {
            if invd[a] < 0.0 {
                std::mem::swap(&mut t0[a], &mut t1[a]);
            }
            t0_min = if t0[a] > t0_min { t0[a] } else { t0_min };
            t1_max = if t1[a] > t1_max { t1[a] } else { t1_max };
            if t1_max <= t0_min {
                return false;
            }
        }
        return true;
    }

    pub fn union(hittables: &[& dyn Hittable]) -> Option<AABB> {
        fn do_union(a: AABB, b: AABB) -> AABB {
            let min = Vec3::new(a.min.x.min(b.min.x),
                                a.min.y.min(b.min.y),
                                a.min.z.min(b.min.z));
            let max = Vec3::new(a.max.x.max(b.max.x),
                                a.max.y.max(b.max.y),
                                a.max.z.max(b.max.z));
            AABB::new(min, max)
        };
        let items = hittables.into_iter();
        let mut res = items.filter_map(|hittable| hittable.get_aabb()).peekable();
        if res.peek().is_none() {
            return None;
        }
        let aabb = res.fold(AABB::zero(), do_union);
        return Some(aabb);
    }

    pub fn get_longest_axis(&self) -> Axis {
        let size = (self.max - self.min).abs();
        if size.x > size.y && size.x > size.z {
            Axis::X
        } else if size.y > size.z {
            Axis::Y
        } else {
            Axis::Z
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_single_sphere() {
        use crate::Sphere;
        use crate::material::Lambertian;
        use crate::Vec3;
        use crate::hittable::Hittable;

        let mat = Lambertian::new(Vec3::new(0.2, 0.3, 0.7));

        let sp = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0, &mat);
        let aabb = sp.get_aabb().unwrap();

        assert_eq!(aabb.max, Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(aabb.min, Vec3::new(-1.0, -1.0, -1.0));
    }

    #[test]
    fn test_union() {
        use crate::Sphere;
        use crate::material::Lambertian;
        use crate::Vec3;
        use crate::hittable::Hittable;
        use crate::aabb::AABB;

        let mat = Lambertian::new(Vec3::new(0.2, 0.3, 0.7));

        let sp0 = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0, &mat);
        let sp1 = Sphere::new(Vec3::new(1.0, 0.0, 0.0), 1.0, &mat);
        let aabb0 = sp0.get_aabb().unwrap();
        let aabb1 = sp1.get_aabb().unwrap();
        let aabb = AABB::union(&aabb0, &aabb1);

        assert_eq!(aabb.max, Vec3::new(2.0, 1.0, 1.0));
        assert_eq!(aabb.min, Vec3::new(-1.0, -1.0, -1.0));
    }
}
