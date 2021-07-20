use nalgebra as na;
use num::ToPrimitive;

use crate::ray::Ray;

pub trait Intersectable<T, U>
where
    T: na::RealField,
    U: na::RealField + ToPrimitive,
{
    fn intersect(&self, ray: &Ray<T>) -> Option<T>;
    fn color(&self) -> na::Vector3<U>;
}
