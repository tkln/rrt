use std::cmp::Ordering;
use std::fmt;

use crate::Vec3;
use crate::Ray;
use crate::hittable::{Hittable, HitRecord, HittableList};
use crate::rng::RNG;

#[derive(Copy, Clone)]
enum Axis {
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

    fn get_longest_axis(&self) -> Axis {
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

pub enum BVH<'a> {
    Node {
        left: Box::<BVH<'a>>,
        right: Box::<BVH<'a>>,
        aabb: AABB,
    },
    Leaf {
        hittable: &'a dyn Hittable,
    }
}

impl BVH<'_> {
    pub fn new<'a>(items: Vec<&'a dyn Hittable>) -> BVH<'a> {
        let mut objs = items;
        let span = objs.len();

        if span == 1 {
            return BVH::Leaf {
                hittable: objs[0],
            };
        }

        let aabb = AABB::union(&objs).unwrap();

        let axis = aabb.get_longest_axis();
        let cmp = |a: AABB, b: AABB| -> std::cmp::Ordering {
            let idx = axis as usize;
            let va = a.min[idx];
            let vb = b.min[idx];
            if va == vb {
                Ordering::Equal
            } else if va > vb {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        };

        if span == 2 {
            let a = Box::new(BVH::Leaf { hittable: objs[0] });
            let b = Box::new(BVH::Leaf { hittable: objs[1] });
            if cmp(objs[0].get_aabb().unwrap(),
                   objs[1].get_aabb().unwrap()) == Ordering::Less {
                return BVH::Node {
                    left: a,
                    right: b,
                    aabb: aabb,
                };
            } else {
                return BVH::Node {
                    left: b,
                    right: a,
                    aabb: aabb,
                };
            }
        } else {
            objs.sort_by(|a, b| cmp(a.get_aabb().unwrap(),
                                    b.get_aabb().unwrap()));
            let partition = span / 2;

            return BVH::Node {
                left: Box::new(BVH::new(objs[..partition].to_vec())),
                right: Box::new(BVH::new(objs[partition..].to_vec())),
                aabb: aabb,
            };
        }
    }
}

impl Hittable for BVH<'_> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rng: &mut RNG) -> Option<HitRecord> {
        match self {
            BVH::Node { left, right, aabb } =>
            {
                if !aabb.hit(ray, t_min, t_max) {
                    return None
                }

                let hit_left = left.hit(ray, t_min, t_max, rng);
                let hit_right = right.hit(ray, t_min, t_max, rng);

                match (hit_left, hit_right) {
                    (Some(l), Some(r))  => Some(if l.t < r.t { l } else { r }),
                    (None, Some(r))     => Some(r),
                    (Some(l), None)     => Some(l),
                    (None, None)        => None,
                }
            }

            BVH::Leaf { hittable } => hittable.hit(ray, t_min, t_max, rng),
        }
    }

    fn get_aabb(&self) -> Option<AABB> {
        match self {
            BVH::Node { left: _, right: _, aabb } => Some(*aabb),
            BVH::Leaf { hittable } => hittable.get_aabb(),
        }
    }
}

impl fmt::Debug for BVH<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            BVH::Node { left, right, aabb }=>
            f.debug_struct("Node")
             .field("left", &left)
             .field("right", &right)
             .field("aabb", &aabb)
             .finish(),
            BVH::Leaf {hittable: _} =>
            f.debug_struct("Leaf").finish(),
        }
    }
}
