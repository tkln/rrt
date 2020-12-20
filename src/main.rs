use std::mem;

mod vec3;
use vec3::Vec3;

fn main() {
    println!("Hello, world!");
    println!("{}", mem::align_of::<Vec3>());
    let a = Vec3::new(1.0, 2.0, 3.0);
    let mut b = Vec3::one();
    b += a;
}
