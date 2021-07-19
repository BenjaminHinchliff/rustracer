use nalgebra::RealField;

use crate::sphere::Sphere;

pub struct Scene<T: RealField> {
    pub width: u32,
    pub height: u32,
    pub fov: T,
    pub sphere: Sphere<T>,
}
