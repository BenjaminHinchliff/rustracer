use nalgebra as na;

mod ray;
mod render;
mod scene;
mod sphere;
mod intersectable;

use render::render;
use scene::Scene;
use sphere::Sphere;

fn main() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
            center: na::Point3::new(0.0, 0.0, -5.0),
            radius: 1.0,
            color: na::Vector3::new(0.4, 1.0, 0.4),
        },
    };

    let img = render(&scene);
    img.save("render.png").expect("failed to save render");
}
