use std::sync::{Arc, Mutex};

use image::RgbImage;
use nalgebra as na;
use num::ToPrimitive;
use threadpool::ThreadPool;

use crate::{
    color_convert::vec3_to_rgb, intersection::Intersection, material::SurfaceType,
    objects::Intersectable, ray::Ray, scene::Scene,
};

fn fresnel<T>(incident: na::Vector3<T>, normal: na::Vector3<T>, index: T) -> T
where
    T: na::RealField + ToPrimitive,
{
    let i_dot_n = incident.dot(&normal);
    let mut eta_i = T::one();
    let mut eta_t = index;
    if i_dot_n > T::zero() {
        eta_i = eta_t;
        eta_t = T::one();
    }

    let sin_t = eta_i / eta_t * (T::one() - i_dot_n * i_dot_n).max(T::zero()).sqrt();
    if sin_t > T::one() {
        T::one()
    } else {
        let cos_t = (T::one() - sin_t * sin_t).max(T::zero()).sqrt();
        let cos_i = cos_t.abs();
        let r_s = ((eta_t * cos_i) - (eta_i * cos_t)) / ((eta_t * cos_i) + (eta_i * cos_t));
        let r_p = ((eta_i * cos_i) - (eta_t * cos_t)) / ((eta_i * cos_i) + (eta_t * cos_t));
        (r_s * r_s + r_p * r_p) / T::from_f64(2.0).unwrap()
    }
}

fn shade_diffuse<T>(
    scene: &Scene<T>,
    object: &dyn Intersectable<T>,
    hit_point: na::Point3<T>,
    surface_normal: na::Vector3<T>,
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
    let normal = intersection.object.surface_normal(&hit_point);

    let material = intersection.object.material();
    match material.surface {
        SurfaceType::Diffuse => shade_diffuse(scene, intersection.object, hit_point, normal),
        SurfaceType::Reflective { reflectivity } => {
            let mut color = shade_diffuse(scene, intersection.object, hit_point, normal);

            let reflection_ray =
                Ray::create_reflection(normal, ray.direction, hit_point, scene.shadow_bias);

            color *= T::one() - reflectivity;
            color += cast_ray(scene, &reflection_ray, depth + 1) * reflectivity;
            color
        }
        SurfaceType::Refractive {
            index,
            transparency,
        } => {
            let mut refraction_color = na::Vector3::zeros();
            let kr = fresnel(ray.direction, normal, index);
            let surface_color = material
                .color
                .color(&intersection.object.texture_coords(&hit_point));

            if kr < T::one() {
                let transmission_ray = Ray::create_transmission(
                    normal,
                    ray.direction,
                    hit_point,
                    scene.shadow_bias,
                    index,
                )
                .unwrap();
                refraction_color = cast_ray(scene, &transmission_ray, depth + 1);
            }

            let reflection_ray =
                Ray::create_reflection(normal, ray.direction, hit_point, scene.shadow_bias);
            let reflection_color = cast_ray(scene, &reflection_ray, depth + 1);
            let mut color = reflection_color * kr + refraction_color * (T::one() - kr);
            color.component_mul_assign(&(surface_color * transparency));
            color
        }
    }
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
