use nalgebra as na;
use num::ToPrimitive;

use crate::{intersectable::Intersectable, ray::Ray};

pub struct Sphere<T: na::RealField, U: na::RealField + ToPrimitive> {
    pub center: na::Point3<T>,
    pub radius: T,
    pub color: na::Vector3<U>,
}

impl<T: na::RealField, U: na::RealField + ToPrimitive> Intersectable<T> for Sphere<T, U> {
    fn intersect(&self, ray: &Ray<T>) -> Option<T> {
        let l = self.center - ray.origin;
        let adj = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj * adj);
        let r2 = self.radius * self.radius;
        if d2 > r2 {
            return None;
        }

        let thc = (r2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < T::zero() && t1 < T::zero() {
            return None;
        }

        let distance = t0.min(t1);
        Some(distance)
    }
}
