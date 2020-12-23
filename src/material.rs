use crate::Vec3;
use crate::Ray;
use crate::hittable::HitRecord;
use crate::rng::*;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, rng: &mut RNG) -> Option<(Vec3, Ray)>;
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord, rng: &mut RNG) -> Option<(Vec3, Ray)> {
        let scatter_dir = rec.n + random_unit_vector(rng);
        let scattered = Ray::new(rec.p, scatter_dir);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, rng: &mut RNG) -> Option<(Vec3, Ray)> {
        let reflected = Vec3::reflect(ray_in.dir.normalized(), rec.n);
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        if scattered.dir.dot(rec.n) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}


