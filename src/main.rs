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
    println!("Hello, world!");
    println!("{}", mem::align_of::<Vec3>());
    let a = Vec3::new(1.0, 2.0, 3.0);
    let mut b = Vec3::one();
    b += a;
}
