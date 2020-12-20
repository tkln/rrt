use std::mem;

mod vec3;
use vec3::Vec3;

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

fn main() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let mut b = Vec3::one();
    b += a;

    let w = 256;
    let h = 256;
    let mut img = vec![Vec3::zero(); w * h];

    for x in 0..h {
        for y in 0..w {
            img[x + y * w] = Vec3::new((x as f32) / (w as f32),
                                       (y as f32) / (h as f32),
                                       0.25);
        }
    }

    save_image(w, h, &img);
}
