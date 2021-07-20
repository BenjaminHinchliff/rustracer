use nalgebra as na;
use num::ToPrimitive;

use crate::light::Light;

#[derive(Debug)]
pub struct DirectionalLight<T>
where
    T: na::RealField + ToPrimitive,
{
    pub direction: na::Vector3<T>,
    pub color: na::Vector3<T>,
    pub intensity: T,
}

impl<T> Light<T> for DirectionalLight<T>
where
    T: na::RealField + ToPrimitive,
{
    fn color(&self) -> na::Vector3<T> {
        self.color
    }

    fn direction_from(&self, _hit_point: &na::Point3<T>) -> na::Vector3<T> {
        -self.direction.normalize()
    }

    fn intensity(&self, _hit_point: &na::Point3<T>) -> T {
        self.intensity
    }

    fn distance(&self, _hit_point: &na::Point3<T>) -> T {
        T::one() / T::zero() // infinity
    }
}
