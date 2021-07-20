use nalgebra as na;
use num::ToPrimitive;

use crate::{intersectable::Intersectable, ray::Ray};

pub struct Sphere<T>
where
    T: na::RealField + ToPrimitive,
{
    pub center: na::Point3<T>,
    pub radius: T,
    pub albedo: T,
    pub color: na::Vector3<T>,
}

impl<T> Intersectable<T> for Sphere<T>
where
    T: na::RealField + ToPrimitive,
{
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

    fn surface_normal(&self, hit_point: &na::Point3<T>) -> na::Vector3<T> {
        (*hit_point - self.center).normalize()
    }

    fn albedo(&self) -> T {
        self.albedo
    }

    fn color(&self) -> na::Vector3<T> {
        self.color
    }
}
