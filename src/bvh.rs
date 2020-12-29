use std::cmp::Ordering;
use std::fmt;

use crate::Ray;
use crate::hittable::{Hittable, HitRecord, HittableList};
use crate::rng::RNG;
use crate::aabb::AABB;

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

                let hit_left = &left.hit(ray, t_min, t_max, rng);
                let hit_right = &right.hit(ray, t_min, t_max, rng);

                match (hit_left, hit_right) {
                    (Some(l), Some(r))  => Some(if l.t < r.t { *l } else { *r }),
                    (None, Some(r))     => Some(*r),
                    (Some(l), None)     => Some(*l),
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
