use nalgebra as na;
use num::ToPrimitive;

use crate::scene::Scene;

pub struct Ray<T: na::RealField + ToPrimitive> {
    pub origin: na::Point3<T>,
    pub direction: na::Vector3<T>,
}

fn to_radians<T: na::RealField + ToPrimitive>(a: T) -> T {
    a * T::pi() / na::convert(180.0)
}

impl<T> Ray<T>
where
    T: na::RealField + ToPrimitive,
{
    pub fn new_prime(x: u32, y: u32, scene: &Scene<T>) -> Ray<T> {
        let Scene {
            width, height, fov, ..
        } = *scene;
        assert!(
            width > height,
            "width must be greater than height to prevent distortion (for now)"
        );

        let (width, height) = (T::from_u32(width).unwrap(), T::from_u32(height).unwrap());
        let half = na::convert(0.5);
        let two = na::convert(2.0);

        let aspect = width / height;
        let fov_adj = (to_radians(fov) / two).tan();

        let x = T::from_u32(x).unwrap();
        let sensor_x = (((x + half) / width) * two - T::one()) * aspect * fov_adj;
        let y = T::from_u32(y).unwrap();
        let sensor_y = T::one() - ((y + half) / height) * two;

        Ray {
            origin: na::Point3::origin(),
            direction: na::Vector3::new(sensor_x, sensor_y, -T::one()).normalize(),
        }
    }
}
