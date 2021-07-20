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

fn cast_ray<T, U>(scene: &Scene<T, U>, ray: &Ray<T>) -> Rgb<u8>
where
    T: na::RealField,
    U: na::RealField + ToPrimitive,
{
    let intersection = scene.trace(ray);
    intersection
        .map(|i| vec3_to_rgb(i.object.color))
        .unwrap_or(BLACK)
}

pub fn render<T, U>(scene: &Scene<T, U>) -> RgbImage
where
    T: na::RealField,
    U: na::RealField + ToPrimitive,
{
    let Scene { width, height, .. } = *scene;

    let mut img = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let ray = Ray::new_prime(x, y, scene);

            let color = cast_ray(scene, &ray);
            img.put_pixel(x, y, color);
        }
    }
    img
}
