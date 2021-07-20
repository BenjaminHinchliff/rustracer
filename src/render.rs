use std::sync::{Arc, Mutex};

use image::{Rgb, RgbImage};
use nalgebra as na;
use num::ToPrimitive;
use threadpool::ThreadPool;

use crate::{intersection::Intersection, ray::Ray, scene::Scene};

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

    let mut color = na::Vector3::zeros();
    for light in &scene.lights {
        let dir_to_light = light.direction_from(&hit_point);

        let shadow_ray = Ray {
            origin: hit_point + (surface_normal * scene.shadow_bias),
            direction: dir_to_light,
        };
        let shadow_intersection = scene.trace(&shadow_ray);
        let in_light = shadow_intersection.is_none()
            || shadow_intersection.unwrap().distance > light.distance(&hit_point);

        let light_intensity = if in_light {
            light.intensity(&hit_point)
        } else {
            T::zero()
        };

        let material = intersection.object.material();

        let light_power = (surface_normal.dot(&dir_to_light)).max(T::zero()) * light_intensity;
        let light_reflected = material.albedo / T::pi();

        let light_color = light.color() * light_power * light_reflected;
        color += material.color.component_mul(&light_color);
    }

    color.apply_into(|e| e.clamp(T::zero(), T::one()))
}

fn cast_ray<T>(scene: &Scene<T>, ray: &Ray<T>) -> na::Vector3<T>
where
    T: na::RealField + ToPrimitive,
{
    let intersection = scene.trace(ray);
    intersection
        .map(|i| calculate_color(scene, ray, &i))
        .unwrap_or(na::Vector3::zeros())
}

pub fn render<T>(scene: Scene<T>) -> RgbImage
where
    T: na::RealField + ToPrimitive,
{
    let Scene {
        width,
        height,
        samples,
        ..
    } = scene;

    let img = Arc::new(Mutex::new(RgbImage::new(width, height)));
    let scene = Arc::new(scene);
    let pool = ThreadPool::new(num_cpus::get());

    for x in 0..width {
        for y in 0..height {
            let img = img.clone();
            let scene = scene.clone();
            pool.execute(move || {
                let mut color = na::Vector3::zeros();
                for s in 0..samples {
                    let ray = Ray::new_prime(x, y, s, &scene);

                    color += cast_ray(&scene, &ray);
                }
                color /= T::from_u32(samples).unwrap();

                let mut img = img.lock().unwrap();
                img.put_pixel(x, y, vec3_to_rgb(color));
            });
        }
    }

    pool.join();
    Arc::try_unwrap(img).unwrap().into_inner().unwrap()
}
