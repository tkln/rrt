use std::ops;

#[repr(align(16))] /* Align for SSE */
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3{x: x, y: y, z: z}
    }

    pub fn zero() -> Vec3 {
        Vec3{x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn one() -> Vec3 {
        Vec3{x: 1.0, y: 1.0, z: 1.0}
    }

    pub fn len2(&self) -> f32 {
        self.x * self.x +
        self.y * self.y +
        self.z * self.z
    }

    pub fn len(&self) -> f32 {
        self.len2().sqrt()
    }

    pub fn normalized(&self) -> Vec3 {
        /* TODO Use fast reciprocal squareroot */
        *self / self.len()
    }

    pub fn dot(&self, rhs: Vec3) -> f32 {
        let s = *self * rhs;
        s.x + s.y + s.z
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }

    pub fn sqrt(self) -> Self {
        Vec3 {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }

    pub fn reflect(v: Self, n: Self) -> Self {
        v - n * 2.0 * v.dot(n)
    }

    pub fn refract(uv: Self, n: Self, r_eta: f32) -> Self {
        let cos_theta = (-uv).dot(n).min(1.0);
        let r_out_perp = (uv + n * cos_theta ) * r_eta;
        let r_out_parallel = -n * (1.0 - r_out_perp.len2()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 { x: -self.x,
               y: -self.y,
               z: -self.z }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x + rhs.x,
               y: self.y + rhs.y,
               z: self.z + rhs.z }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x - rhs.x,
               y: self.y - rhs.y,
               z: self.z - rhs.z }
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = *self - rhs;
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x * rhs.x,
               y: self.y * rhs.y,
               z: self.z * rhs.z }
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = *self * rhs;
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 { x: self.x * rhs,
               y: self.y * rhs,
               z: self.z * rhs }
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3 {
        Vec3 { x: self.x / rhs,
               y: self.y / rhs,
               z: self.z / rhs }
    }
}
