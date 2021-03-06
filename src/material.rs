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
        let fuzz_dir = random_in_unit_sphere(rng) * self.fuzz;
        let scattered = Ray::new(rec.p, reflected + fuzz_dir);
        let attenuation = self.albedo;
        if scattered.dir.dot(rec.n) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    ir: f32,
}

impl Dielectric {
    pub fn new(ir: f32) -> Dielectric {
        Dielectric{ir}
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let r = (1.0 - ref_idx) / (1.0 + ref_idx);
        let rr = r * r;
        rr + (1.0 - rr) * f32::powf(1.0 - cosine, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, rng: &mut RNG) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3::one();
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_dir = ray_in.dir.normalized();

        let cos_theta = (-unit_dir).dot(rec.n).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let no_refract = refraction_ratio * sin_theta > 1.0;
        let reflectance = Self::reflectance(cos_theta, refraction_ratio);

        let dir = if no_refract || reflectance > random_f32(rng) {
            Vec3::reflect(unit_dir, rec.n)
        } else {
            Vec3::refract(unit_dir, rec.n, refraction_ratio)
        };

        let scattered = Ray::new(rec.p, dir);
        Some((attenuation, scattered))
    }
}
