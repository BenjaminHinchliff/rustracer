use nalgebra as na;

mod intersectable;
mod intersection;
mod plane;
mod ray;
mod render;
mod scene;
mod sphere;

use render::render;
use scene::Scene;
use sphere::Sphere;

use crate::{plane::Plane, scene::Light};

fn main() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        objects: vec![
            Box::new(Sphere {
                center: na::Point3::new(0.0, 0.0, -5.0),
                radius: 1.0,
                color: na::Vector3::new(0.4, 1.0, 0.4),
                albedo: 0.18,
            }),
            Box::new(Sphere {
                center: na::Point3::new(-3.0, 1.0, -6.0),
                radius: 2.0,
                color: na::Vector3::new(0.2, 0.2, 1.0),
                albedo: 0.58,
            }),
            Box::new(Sphere {
                center: na::Point3::new(2.0, 2.0, -4.0),
                radius: 2.25,
                color: na::Vector3::new(1.0, 0.2, 0.2),
                albedo: 0.08,
            }),
            Box::new(Plane {
                origin: na::Point3::new(0.0, 0.0, -20.0),
                normal: na::Vector3::new(0.0, 0.0, -1.0),
                color: na::Vector3::new(0.6, 0.8, 1.0),
                albedo: 0.18,
            }),
            Box::new(Plane {
                origin: na::Point3::new(0.0, -2.0, 0.0),
                normal: na::Vector3::new(0.0, -1.0, 0.0),
                color: na::Vector3::new(0.2, 0.2, 0.2),
                albedo: 0.18,
            }),
        ],
        light: Light {
            direction: na::Vector3::new(-0.25, -1.0, -1.0),
            color: na::Vector3::new(1.0, 1.0, 1.0),
            intensity: 20.0,
        },
        shadow_bias: 1e-13,
    };

    let img = render(&scene);
    img.save("render.png").expect("failed to save render");
}
