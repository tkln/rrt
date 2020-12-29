use crate::Vec3;
use crate::Ray;

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
        assert!(min.x < max.x && min.y < max.y && min.z < max.z);
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

    pub fn union(box_a: &AABB, box_b: &AABB) -> AABB {
        let min = Vec3::new(box_a.min.x.min(box_b.min.x),
                            box_a.min.y.min(box_b.min.y),
                            box_a.min.z.min(box_b.min.z));
        let max = Vec3::new(box_a.max.x.max(box_b.max.x),
                            box_a.max.y.max(box_b.max.y),
                            box_a.max.z.max(box_b.max.z));
        AABB::new(min, max)
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


