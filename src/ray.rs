use nalgebra as na;
use num::{integer::Roots, ToPrimitive};

use crate::scene::Scene;

#[derive(Debug)]
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
    pub fn new_prime(x: u32, y: u32, s: u32, scene: &Scene<T>) -> Ray<T> {
        let Scene {
            width, height, fov, ..
        } = *scene;
        assert!(
            width > height,
            "width must be greater than height to prevent distortion (for now)"
        );

        let (width, height) = (T::from_u32(width).unwrap(), T::from_u32(height).unwrap());
        let two = na::convert(2.0);

        let aspect = width / height;
        let fov_adj = (to_radians(fov) / two).tan();

        let ss = scene.samples.sqrt();
        let sw = T::one() / T::from_u32(ss).unwrap();
        let sw_2 = sw / two;
        let sx = T::from_u32(s % ss).unwrap() * sw + sw_2;
        let sy = T::from_u32(s / ss).unwrap() * sw + sw_2;

        let x = T::from_u32(x).unwrap();
        let sensor_x = (((x + sx) / width) * two - T::one()) * aspect * fov_adj;
        let y = T::from_u32(y).unwrap();
        let sensor_y = T::one() - ((y + sy) / height) * two;

        Ray {
            origin: na::Point3::origin(),
            direction: na::Vector3::new(sensor_x, sensor_y, -T::one()).normalize(),
        }
    }

    pub fn create_reflection(
        normal: na::Vector3<T>,
        incident: na::Vector3<T>,
        intersection: na::Point3<T>,
        bias: T,
    ) -> Ray<T> {
        Ray {
            origin: intersection + (normal * bias),
            direction: incident - (normal * T::from_f64(2.0).unwrap() * incident.dot(&normal)),
        }
    }

    pub fn create_transmission(
        normal: na::Vector3<T>,
        incident: na::Vector3<T>,
        intersection: na::Point3<T>,
        bias: T,
        index: T,
    ) -> Option<Ray<T>> {
        let mut ref_n = normal;
        let mut eta_t = index;
        let mut eta_i = T::one();
        let mut i_dot_n = incident.dot(&normal);
        if i_dot_n < T::zero() {
            i_dot_n = -i_dot_n;
        } else {
            ref_n = -normal;
            eta_t = T::one();
            eta_i = index;
        }

        let eta = eta_i / eta_t;
        let k = T::one() - (eta * eta) * (T::one() - i_dot_n * i_dot_n);
        if k < T::zero() {
            None
        } else {
            Some(Ray {
                origin: intersection + (ref_n * -bias),
                direction: (incident + ref_n * i_dot_n) * eta - ref_n * k.sqrt(),
            })
        }
    }
}
