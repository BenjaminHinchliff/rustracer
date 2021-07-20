use nalgebra as na;
use num::ToPrimitive;

use crate::{intersectable::Intersectable, intersection::Intersection, ray::Ray, sphere::Sphere};

pub struct Scene<T, U>
where
    T: na::RealField,
    U: na::RealField + ToPrimitive,
{
    pub width: u32,
    pub height: u32,
    pub fov: T,
    pub spheres: Vec<Sphere<T, U>>,
}

impl<T, U> Scene<T, U>
where
    T: na::RealField,
    U: na::RealField + ToPrimitive,
{
    pub fn trace(&self, ray: &Ray<T>) -> Option<Intersection<T, U>> {
        self.spheres
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
    }
}
