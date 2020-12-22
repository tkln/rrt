mod vec3;
mod ray;
mod sphere;
mod hittable;

use ray::Ray;
use vec3::Vec3;
use sphere::Sphere;
use hittable::Hittable;

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

fn get_color(ray: &Ray) -> Vec3 {
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);

    if let Some(rec) = sphere.hit(ray, 0.0, 999.0) {
        return (rec.n + Vec3::one()) * 0.5;
    }

    /* Fake sky */
    let unit_dir = ray.dir.normalized();
    let t = 0.5 * (unit_dir.y + 1.0);
    Vec3::one() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let img_ar = 16.0 / 9.0;
    let img_w = 400;
    let img_h = (img_w as f32  / img_ar) as usize;
    let mut img = vec![Vec3::zero(); img_w * img_h];

    let viewport_h = 2.0;
    let viewport_w = img_ar * viewport_h;
    let focal_len = 1.0;

    let orig = Vec3::new(0.0, 0.0, 0.0);
    let horiz = Vec3::new(viewport_w, 0.0, 0.0);
    let vert = Vec3::new(0.0, viewport_h, 0.0);
    let lower_left = orig - horiz * 0.5 - vert * 0.5 -
                     Vec3::new(0.0, 0.0, focal_len);

    for y in 0..img_h {
        for x in 0..img_w {
            let u = (x as f32) / ((img_w - 1) as f32);
            let v = (y as f32) / ((img_h - 1) as f32);
            let ray = Ray::new(orig, lower_left + horiz * u + vert * v - orig);
            img[x + y * img_w] = get_color(&ray);
        }
    }

    save_image(img_w, img_h, &img);
}
