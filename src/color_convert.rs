use nalgebra as na;
use image::{Rgb, Rgba};
use num::ToPrimitive;

pub fn vec3_to_rgb<T: na::RealField + ToPrimitive>(mut vec: na::Vector3<T>) -> Rgb<u8> {
    let u8_max = T::from_u8(u8::MAX).unwrap();
    vec *= u8_max;
    Rgb([
        vec.x.to_u8().unwrap(),
        vec.y.to_u8().unwrap(),
        vec.z.to_u8().unwrap(),
    ])
}

// drops alpha component
pub fn rgba_to_vec3<T: na::RealField + ToPrimitive>(rgba: &Rgba<u8>) -> na::Vector3<T> {
    let u8_max = T::from_u8(u8::MAX).unwrap();
	let x = T::from_u8(rgba[0]).unwrap() / u8_max;
	let y = T::from_u8(rgba[1]).unwrap() / u8_max;
	let z = T::from_u8(rgba[2]).unwrap() / u8_max;
	na::Vector3::new(x, y, z)
}
