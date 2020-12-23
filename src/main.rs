mod vec3;
mod ray;
mod sphere;
mod hittable;

use ray::Ray;
use vec3::Vec3;
use sphere::Sphere;
use hittable::{Hittable, HittableList};

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

fn get_color(ray: &Ray, hittables: &HittableList) -> Vec3 {
    if let Some(rec) = hittables.hit(ray, 0.0, 999.0) {
        return (rec.n + Vec3::one()) * 0.5;
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

    let cam = Camera::new(img_ar);

    let hittables = HittableList {
        hittables: vec![
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
        ],
    };

    for y in 0..img_h {
        for x in 0..img_w {
            let u = (x as f32) / ((img_w - 1) as f32);
            let v = ((img_h - y) as f32) / ((img_h - 1) as f32);
            let ray = cam.get_ray(u, v);
            img[x + y * img_w] = get_color(&ray, &hittables);
        }
    }

    save_image(img_w, img_h, &img);
}
