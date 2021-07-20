use nalgebra as na;
use num::ToPrimitive;

use crate::{intersectable::Intersectable, material::Material, ray::Ray};

#[derive(Debug)]
pub struct Plane<T>
where
    T: na::RealField + ToPrimitive,
{
    pub origin: na::Point3<T>,
    pub normal: na::Vector3<T>,
    pub material: Material<T>,
}

impl<T> Intersectable<T> for Plane<T>
where
    T: na::RealField + ToPrimitive,
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

    fn surface_normal(&self, _: &na::Point3<T>) -> na::Vector3<T> {
        -self.normal
    }

    fn material(&self) -> &Material<T> {
        &self.material
    }
}
