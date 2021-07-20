use std::fmt::Debug;

use nalgebra as na;
use num::ToPrimitive;

use crate::{material::Material, ray::Ray};

pub trait Intersectable<T>: Debug + Sync + Send
where
    T: na::RealField + ToPrimitive,
{
    fn intersect(&self, ray: &Ray<T>) -> Option<T>;
    fn surface_normal(&self, hit_point: &na::Point3<T>) -> na::Vector3<T>;
    fn material(&self) -> &Material<T>;
}
