use nalgebra as na;
use num::ToPrimitive;

use crate::sphere::Sphere;

pub struct Scene<T: na::RealField, U: na::RealField + ToPrimitive> {
    pub width: u32,
    pub height: u32,
    pub fov: T,
    pub sphere: Sphere<T, U>,
}
