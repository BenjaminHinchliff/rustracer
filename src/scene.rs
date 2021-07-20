use nalgebra as na;
use num::ToPrimitive;

use crate::{intersectable::Intersectable, intersection::Intersection, ray::Ray};

pub struct Scene<T, U>
where
    T: na::RealField,
    U: na::RealField + ToPrimitive,
{
    pub width: u32,
    pub height: u32,
    pub fov: T,
    pub objects: Vec<Box<dyn Intersectable<T, U>>>,
}

impl<T, U> Scene<T, U>
where
    T: na::RealField,
    U: na::RealField + ToPrimitive,
{
    pub fn trace(&self, ray: &Ray<T>) -> Option<Intersection<T, U>> {
        self.objects
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s.as_ref())))
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
    }
}
