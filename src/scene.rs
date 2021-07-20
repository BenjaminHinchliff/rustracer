use nalgebra as na;
use num::ToPrimitive;

use crate::{intersectable::Intersectable, intersection::Intersection, light::Light, ray::Ray};

#[derive(Debug)]
pub struct Scene<T>
where
    T: na::RealField + ToPrimitive,
{
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    pub fov: T,
    pub objects: Vec<Box<dyn Intersectable<T>>>,
    pub lights: Vec<Box<dyn Light<T>>>,
    pub shadow_bias: T,
}

impl<T> Scene<T>
where
    T: na::RealField + ToPrimitive,
{
    pub fn trace(&self, ray: &Ray<T>) -> Option<Intersection<T>> {
        self.objects
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s.as_ref())))
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
    }
}
