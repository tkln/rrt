use crate::Vec3;
use crate::material::Material;

pub struct Tri<'a> {
    pub verts: [Vec3; 3],
    mat: &'a dyn Material,
}

impl Tri<'_> {
    pub fn new(verts: [Vec3; 3], mat: &dyn Material) -> Tri {
        Tri { verts, mat }
    }
}
