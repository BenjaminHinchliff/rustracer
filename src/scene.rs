use nalgebra as na;
use num::ToPrimitive;

use crate::{intersectable::Intersectable, intersection::Intersection, ray::Ray};

pub struct Light<T>
where
    T: na::RealField + ToPrimitive,
{
    pub direction: na::Vector3<T>,
    pub color: na::Vector3<T>,
    pub intensity: T,
}

pub struct Scene<T>
where
    T: na::RealField + ToPrimitive,
{
    pub width: u32,
    pub height: u32,
    pub fov: T,
    pub objects: Vec<Box<dyn Intersectable<T>>>,
    pub light: Light<T>,
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
