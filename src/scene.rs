use nalgebra as na;

use crate::sphere::Sphere;

pub struct Scene<T: na::RealField> {
    pub width: u32,
    pub height: u32,
    pub fov: T,
    pub sphere: Sphere<T>,
}
