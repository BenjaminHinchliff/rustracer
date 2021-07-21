use nalgebra as na;

mod color_convert;
mod coloration;
mod directional_light;
mod intersectable;
mod intersection;
mod light;
mod material;
mod plane;
mod ray;
mod render;
mod scene;
mod sphere;
mod spherical_light;

use render::render;
use scene::Scene;
use sphere::Sphere;

use crate::{
    coloration::{Color, Texture},
    directional_light::DirectionalLight,
    material::{Material, SurfaceType},
    plane::Plane,
    spherical_light::SphericalLight,
};

fn main() {
    let checkerboard = image::open("checkerboard.png").expect("failed to open checkerboard texture!");

    let scene = Scene {
        width: 3840,
        height: 2160,
        samples: 9,
        fov: 90.0,
        objects: vec![
            Box::new(Sphere {
                center: na::Point3::new(0.0, 0.0, -5.0),
                radius: 1.0,
                material: Material {
                    color: Box::new(Color {
                        color: na::Vector3::new(0.4, 1.0, 0.4),
                    }),
                    surface: SurfaceType::Reflective {
                        reflectivity: 0.7,
                    },
                    albedo: 0.18,
                },
            }),
            Box::new(Sphere {
                center: na::Point3::new(-3.0, 1.0, -6.0),
                radius: 2.0,
                material: Material {
                    color: Box::new(Texture {
                        texture: checkerboard.clone(),
                    }),
                    surface: SurfaceType::Diffuse,
                    albedo: 0.58,
                },
            }),
            Box::new(Sphere {
                center: na::Point3::new(2.0, 2.0, -4.0),
                radius: 2.25,
                material: Material {
                    color: Box::new(Color {
                        color: na::Vector3::new(1.0, 0.2, 0.2),
                    }),
                    surface: SurfaceType::Diffuse,
                    albedo: 0.08,
                },
            }),
            Box::new(Plane {
                origin: na::Point3::new(0.0, 0.0, -20.0),
                normal: na::Vector3::new(0.0, 0.0, -1.0),
                material: Material {
                    color: Box::new(Color {
                        color: na::Vector3::new(0.6, 0.8, 1.0),
                    }),
                    surface: SurfaceType::Diffuse,
                    albedo: 0.18,
                },
            }),
            Box::new(Plane {
                origin: na::Point3::new(0.0, -2.0, 0.0),
                normal: na::Vector3::new(0.0, -1.0, 0.0),
                material: Material {
                    color: Box::new(Texture {
                        texture: checkerboard,
                    }),
                    surface: SurfaceType::Reflective {
                        reflectivity: 0.5,
                    },
                    albedo: 0.18,
                },
            }),
        ],
        lights: vec![
            Box::new(SphericalLight {
                position: na::Point3::new(-2.0, 10.0, -3.0),
                color: na::Vector3::new(0.3, 0.8, 0.3),
                intensity: 10000.0,
            }),
            Box::new(SphericalLight {
                position: na::Point3::new(0.25, 0.0, -2.0),
                color: na::Vector3::new(0.8, 0.3, 0.3),
                intensity: 1000.0,
            }),
            Box::new(DirectionalLight {
                direction: na::Vector3::new(0.0, 0.0, -1.0),
                color: na::Vector3::new(1.0, 1.0, 1.0),
                intensity: 0.0,
            }),
        ],
        shadow_bias: 1e-13,
        max_recursion_depth: 10,
    };

    let img = render(scene);
    img.save("render.png").expect("failed to save render");
}
