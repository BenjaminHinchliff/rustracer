use image::{Rgb, Rgba};
use nalgebra as na;
use num::ToPrimitive;

const GAMMA: f64 = 2.2;

pub fn gamma_encode<T>(linear: na::Vector3<T>) -> na::Vector3<T>
where
    T: na::RealField + ToPrimitive,
{
    linear.apply_into(|l| l.powf(T::one() / T::from_f64(GAMMA).unwrap()))
}

pub fn gamma_decode<T>(encoded: na::Vector3<T>) -> na::Vector3<T>
where
    T: na::RealField + ToPrimitive,
{
    encoded.apply_into(|e| e.powf(T::from_f64(GAMMA).unwrap()))
}

pub fn vec3_to_rgb<T>(mut vec: na::Vector3<T>) -> Rgb<u8>
where
    T: na::RealField + ToPrimitive,
{
    let u8_max = T::from_u8(u8::MAX).unwrap();
    vec = gamma_encode(vec);
    vec *= u8_max;
    Rgb([
        vec.x.to_u8().unwrap(),
        vec.y.to_u8().unwrap(),
        vec.z.to_u8().unwrap(),
    ])
}

// drops alpha component
pub fn rgba_to_vec3<T>(rgba: &Rgba<u8>) -> na::Vector3<T>
where
    T: na::RealField + ToPrimitive,
{
    let u8_max = T::from_u8(u8::MAX).unwrap();
    let x = T::from_u8(rgba[0]).unwrap() / u8_max;
    let y = T::from_u8(rgba[1]).unwrap() / u8_max;
    let z = T::from_u8(rgba[2]).unwrap() / u8_max;
    let vec = na::Vector3::new(x, y, z);
    gamma_decode(vec)
}
