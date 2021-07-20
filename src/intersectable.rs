use nalgebra as na;
use num::ToPrimitive;

use crate::ray::Ray;

pub trait Intersectable<T>
where
    T: na::RealField + ToPrimitive,
{
    fn intersect(&self, ray: &Ray<T>) -> Option<T>;
    fn surface_normal(&self, hit_point: &na::Point3<T>) -> na::Vector3<T>;
    fn albedo(&self) -> T;
    fn color(&self) -> na::Vector3<T>;
}
