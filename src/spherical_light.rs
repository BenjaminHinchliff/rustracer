use nalgebra as na;
use num::ToPrimitive;

use crate::light::Light;

#[derive(Debug)]
pub struct SphericalLight<T>
where
    T: na::RealField + ToPrimitive,
{
    pub position: na::Point3<T>,
    pub color: na::Vector3<T>,
    pub intensity: T,
}

impl<T> Light<T> for SphericalLight<T>
where
    T: na::RealField + ToPrimitive,
{
    fn color(&self) -> na::Vector3<T> {
        self.color
    }

    fn direction_from(&self, hit_point: &na::Point3<T>) -> na::Vector3<T> {
        (self.position - hit_point).normalize()
    }

    fn intensity(&self, hit_point: &na::Point3<T>) -> T {
        let four = T::from_f64(4.0).unwrap();
        let r2 = (self.position - hit_point).norm();
        self.intensity / (four * T::pi() * r2)
    }

    fn distance(&self, hit_point: &na::Point3<T>) -> T {
        (self.position - hit_point).magnitude()
    }
}
