use nalgebra as na;

mod intersectable;
mod intersection;
mod ray;
mod render;
mod scene;
mod sphere;

use render::render;
use scene::Scene;
use sphere::Sphere;

fn main() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        spheres: vec![
            Sphere {
                center: na::Point3::new(0.0, 0.0, -5.0),
                radius: 1.0,
                color: na::Vector3::new(0.4, 1.0, 0.4),
            },
            Sphere {
                center: na::Point3::new(-3.0, 1.0, -6.0),
                radius: 2.0,
                color: na::Vector3::new(0.2, 0.2, 1.0),
            },
            Sphere {
                center: na::Point3::new(2.0, 2.0, -4.0),
                radius: 2.25,
                color: na::Vector3::new(1.0, 0.2, 0.2),
            },
        ],
    };

    let img = render(&scene);
    img.save("render.png").expect("failed to save render");
}
