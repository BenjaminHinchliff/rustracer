use nalgebra as na;
use num::ToPrimitive;

use crate::{intersectable::Intersectable, ray::Ray};

pub struct Plane<T, U>
where
    T: na::RealField,
    U: na::RealField + ToPrimitive,
{
    pub origin: na::Point3<T>,
    pub normal: na::Vector3<T>,
    pub color: na::Vector3<U>,
}

impl<T, U> Intersectable<T, U> for Plane<T, U>
where
    T: na::RealField,
    U: na::RealField + ToPrimitive,
{
    fn intersect(&self, ray: &Ray<T>) -> Option<T> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > na::convert(1e-6) {
            let v = self.origin - ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= T::zero() {
                return Some(distance);
            }
        }
        None
    }

    fn color(&self) -> na::Vector3<U> {
        self.color
    }
}
