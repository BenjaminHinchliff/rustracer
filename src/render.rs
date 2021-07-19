use image::RgbImage;
use nalgebra::RealField;

use crate::scene::Scene;

pub fn render<T: RealField>(scene: &Scene<T>) -> RgbImage {
    RgbImage::new(scene.width, scene.height)
}
