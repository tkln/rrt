mod vec3;
mod ray;
mod sphere;
mod hittable;
mod material;
mod rng;

use ray::Ray;
use vec3::Vec3;
use sphere::Sphere;
use hittable::{Hittable, HittableList};
use rng::*;
use material::{Lambertian, Metal};

fn save_image(w: usize, h: usize, pixels: &[Vec3]) {
    println!("P3");
    println!("{} {}", w, h);
    println!("255");
    for y in 0..h {
        for x in 0..w {
            let p = pixels[x + y * w] * 255.99;
            print!("{} {} {} ", p.x as u32, p.y as u32, p.z as u32);
        }
        println!("");
    }
}

fn trace_ray(ray: &Ray, hittables: &HittableList, rng: &mut RNG, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    if let Some(rec) = hittables.hit(ray, 0.00001, 9999.0, rng) {
        if let Some((attennuation, scattered)) = rec.mat.scatter(ray, &rec, rng) {
            return trace_ray(&scattered, hittables, rng, depth - 1) * attennuation;
        } else {
            return Vec3::zero();
        }
    }

    /* Fake sky */
    let unit_dir = ray.dir.normalized();
    let t = 0.5 * (unit_dir.y + 1.0);
    Vec3::one() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

struct Camera {
    orig: Vec3,
    lower_left: Vec3,
    horiz: Vec3,
    vert: Vec3,
}

impl Camera {
    fn new(ar: f32) -> Camera {
        let viewport_h = 2.0;
        let viewport_w = ar * viewport_h;
        let focal_len = 1.0;

        let orig = Vec3::new(0.0, 0.0, 0.0);
        let horiz = Vec3::new(viewport_w, 0.0, 0.0);
        let vert = Vec3::new(0.0, viewport_h, 0.0);
        let lower_left = orig - horiz * 0.5 - vert * 0.5 -
                         Vec3::new(0.0, 0.0, focal_len);

        Camera { orig: orig, lower_left: lower_left, horiz: horiz, vert: vert }
    }

    fn get_ray(&self, u: f32, v: f32) -> Ray {
        let dir = self.lower_left + self.horiz * u + self.vert * v - self.orig;
        Ray::new(self.orig, dir)
    }
}

fn main() {
    let img_ar = 16.0 / 9.0;
    let img_w = 400;
    let img_h = (img_w as f32  / img_ar) as usize;
    let mut img = vec![Vec3::zero(); img_w * img_h];

    let samples_per_pixel = 100;

    let mut rng = RNG::new();

    let cam = Camera::new(img_ar);

    let lambertian_b = Lambertian::new(Vec3::new(0.2, 0.3, 0.7));
    let lambertian_r = Lambertian::new(Vec3::new(0.7, 0.3, 0.2));
    let metal_g = Metal::new(Vec3::new(0.2, 0.7, 0.3), 0.3);

    let sphere_large = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, &(lambertian_r));
    let sphere0 = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, &(metal_g));
    let sphere1 = Sphere::new(Vec3::new(1.0, 1.0, -1.0), 0.5, &(metal_g));
    let sphere2 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &(lambertian_b));

    let hittables = HittableList {
        hittables: vec![
            &sphere0,
            &sphere1,
            &sphere2,
            &sphere_large,
        ],
    };

    let scale = 1.0 / samples_per_pixel as f32;

    let render_start = std::time::Instant::now();
    for y in 0..img_h {
        eprint!("\r{}", (y as f32 / img_h as f32) * 100.0);
        for x in 0..img_w {
            let sample_pixel = | pixel, _ | -> Vec3 {
                let u = (x as f32 + rng.sample_01()) / ((img_w - 1) as f32);
                let v = ((img_h - y) as f32 + rng.sample_01()) / ((img_h - 1) as f32);
                let ray = cam.get_ray(u, v);
                pixel + trace_ray(&ray, &hittables, &mut rng, 50)
            };
            let sum = (0..samples_per_pixel).fold(Vec3::zero(), sample_pixel);
            let pixel = (sum * scale).sqrt();
            img[x + y * img_w] = pixel;
        }
    }
    let render_finish = std::time::Instant::now();
    let render_time = render_finish - render_start;

    eprintln!("\nDone in {:?}!", render_time);

    save_image(img_w, img_h, &img);
}
