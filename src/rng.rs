use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use rand::distributions::Uniform;

use crate::Vec3;

pub struct RNG {
    side_11: Uniform<f32>,
    side_01: Uniform<f32>,
    rng: SmallRng,
}

impl RNG {
    pub fn new() -> RNG {
        RNG {
            side_11: Uniform::new(-1.0, 1.0),
            side_01: Uniform::new(0.0, 1.0),
            rng: SmallRng::from_entropy(),
        }
    }

    pub fn sample_11(&mut self) -> f32 {
        self.rng.sample(self.side_11)
    }

    pub fn sample_01(&mut self) -> f32 {
        self.rng.sample(self.side_01)
    }
}

pub fn random_in_unit_sphere(rng: &mut RNG) -> Vec3 {
    loop {
        let p = Vec3::new(rng.sample_11(), rng.sample_11(), rng.sample_11());
        if p.len2() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector(rng: &mut RNG) -> Vec3 {
    /* XXX */
    random_in_unit_sphere(rng)
}
