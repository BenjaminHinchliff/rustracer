use std::sync::{Arc, Mutex};

use image::RgbImage;
use nalgebra as na;
use num::ToPrimitive;
use threadpool::ThreadPool;

use crate::{
    color_convert::vec3_to_rgb, intersectable::Intersectable, intersection::Intersection,
    material::SurfaceType, ray::Ray, scene::Scene,
};

fn shade_diffuse<T>(
    scene: &Scene<T>,
    object: &dyn Intersectable<T>,
    hit_point: &na::Point3<T>,
    surface_normal: &na::Vector3<T>,
) -> na::Vector3<T>
where
    T: na::RealField + ToPrimitive,
{
    let tex_coords = object.texture_coords(&hit_point);

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

        let material = object.material();

        let light_power = (surface_normal.dot(&dir_to_light)).max(T::zero()) * light_intensity;
        let light_reflected = material.albedo / T::pi();

        let light_color = light.color() * light_power * light_reflected;
        color += material
            .color
            .color(&tex_coords)
            .component_mul(&light_color);
    }

    color.apply_into(|e| e.clamp(T::zero(), T::one()))
}

fn calculate_color<T>(
    scene: &Scene<T>,
    ray: &Ray<T>,
    intersection: &Intersection<T>,
    depth: u32,
) -> na::Vector3<T>
where
    T: na::RealField + ToPrimitive,
{
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.object.surface_normal(&hit_point);

    let mut color = shade_diffuse(scene, intersection.object, &hit_point, &surface_normal);
    if let SurfaceType::Reflective { reflectivity } = intersection.object.material().surface {
        let reflection_ray =
            Ray::create_reflection(surface_normal, ray.direction, hit_point, scene.shadow_bias);

        color *= T::one() - reflectivity;
        color += cast_ray(scene, &reflection_ray, depth + 1) * reflectivity;
    }

    color
}

fn cast_ray<T>(scene: &Scene<T>, ray: &Ray<T>, depth: u32) -> na::Vector3<T>
where
    T: na::RealField + ToPrimitive,
{
    if depth >= scene.max_recursion_depth {
        return na::Vector3::zeros();
    }

    let intersection = scene.trace(ray);
    intersection
        .map(|i| calculate_color(scene, ray, &i, depth))
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

                    color += cast_ray(&scene, &ray, 0);
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
