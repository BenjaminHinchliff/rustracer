use nalgebra as na;
use num::ToPrimitive;

use crate::{intersectable::Intersectable, ray::Ray};

pub struct Sphere<T: na::RealField, U: na::RealField + ToPrimitive> {
    pub center: na::Point3<T>,
    pub radius: T,
    pub color: na::Vector3<U>,
}

impl<T: na::RealField, U: na::RealField + ToPrimitive> Intersectable<T> for Sphere<T, U> {
    fn intersect(&self, ray: &Ray<T>) -> bool {
        let l = self.center - ray.origin;
        let adj = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj * adj);
        d2 < self.radius * self.radius
    }
}
