use image::{Rgb, RgbImage};
use nalgebra as na;
use num::ToPrimitive;

use crate::{intersectable::Intersectable, ray::Ray, scene::Scene};

const BLACK: Rgb<u8> = Rgb([0, 0, 0]);

fn vec3_to_rgb<T: na::RealField + ToPrimitive>(mut vec: na::Vector3<T>) -> Rgb<u8> {
    let u8_max = T::from_u8(u8::MAX).unwrap();
    vec *= u8_max;
    Rgb([
        vec.x.to_u8().unwrap(),
        vec.y.to_u8().unwrap(),
        vec.z.to_u8().unwrap(),
    ])
}

pub fn render<T: na::RealField, U: na::RealField + ToPrimitive>(scene: &Scene<T, U>) -> RgbImage {
    let Scene { width, height, .. } = *scene;

    let mut img = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let ray = Ray::new_prime(x, y, scene);

            let color = if scene.sphere.intersect(&ray) {
                vec3_to_rgb(scene.sphere.color)
            } else {
                BLACK
            };
            img.put_pixel(x, y, color);
        }
    }
    img
}
