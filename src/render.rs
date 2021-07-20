use image::{Rgb, RgbImage};
use nalgebra as na;
use num::ToPrimitive;

use crate::{intersection::Intersection, ray::Ray, scene::Scene};

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

fn calculate_color<T>(
    scene: &Scene<T>,
    ray: &Ray<T>,
    intersection: &Intersection<T>,
) -> na::Vector3<T>
where
    T: na::RealField + ToPrimitive,
{
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.object.surface_normal(&hit_point);
    let dir_to_light = -scene.light.direction;
    let light_power = (surface_normal.dot(&dir_to_light)).max(T::zero()) * scene.light.intensity;
    let light_reflected = intersection.object.albedo() / T::pi();

    let color = intersection
        .object
        .color()
        .component_mul(&scene.light.color)
        * light_power
        * light_reflected;
    color.apply_into(|e| e.clamp(T::zero(), T::one()))
}

fn cast_ray<T>(scene: &Scene<T>, ray: &Ray<T>) -> Rgb<u8>
where
    T: na::RealField + ToPrimitive,
{
    let intersection = scene.trace(ray);
    intersection
        .map(|i| vec3_to_rgb(calculate_color(scene, ray, &i)))
        .unwrap_or(BLACK)
}

pub fn render<T>(scene: &Scene<T>) -> RgbImage
where
    T: na::RealField + ToPrimitive,
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
