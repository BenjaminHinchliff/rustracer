use nalgebra as na;
use num::ToPrimitive;

use crate::coloration::Coloration;

#[derive(Debug, Clone, PartialEq)]
pub enum SurfaceType<T>
where
    T: na::RealField + ToPrimitive,
{
    Diffuse,
    Reflective { reflectivity: T },
    Refractive { index: T, transparency: T },
}

#[derive(Debug)]
pub struct Material<T>
where
    T: na::RealField + ToPrimitive,
{
    pub color: Box<dyn Coloration<T>>,
    pub surface: SurfaceType<T>,
    pub albedo: T,
}
