use crate::Vec3;
use crate::Ray;
use crate::rng::*;

pub struct Camera {
    orig: Vec3,
    lower_left: Vec3,
    horiz: Vec3,
    vert: Vec3,
    w: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(pos: Vec3, tgt: Vec3, up: Vec3,
               ar: f32, vfov: f32, aperture: f32, focus: f32) -> Camera {
        let theta = (vfov / 180.0) * std::f32::consts::PI;
        let h = (theta / 2.0).tan();
        let viewport_h = 2.0 * h;
        let viewport_w = ar * viewport_h;

        let w = (pos - tgt).normalized();
        let u = up.cross(w).normalized();
        let v = w.cross(u);

        let orig = pos;
        let horiz = u * viewport_w * focus;
        let vert = v * viewport_h * focus;
        let lower_left = orig - horiz * 0.5 - vert * 0.5 - w * focus;

        let lens_radius = aperture / 2.0;

        Camera { orig: orig, lower_left: lower_left,
                 horiz: horiz, vert: vert,
                 w: w, u: u, v: v,
                 lens_radius: lens_radius }
    }

    pub fn get_ray(&self, s: f32, t: f32, rng: &mut RNG) -> Ray {
        let rd = random_in_unit_disk(rng) * self.lens_radius ;
        let offset = self.u * rd.x + self.v * rd.y;

        let dir = self.lower_left + self.horiz * s + self.vert * t - self.orig - offset;
        Ray::new(self.orig + offset, dir)
    }
}
