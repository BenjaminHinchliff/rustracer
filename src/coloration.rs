use std::fmt::Debug;

use image::{DynamicImage, GenericImageView};
use na::RealField;
use nalgebra as na;
use num::ToPrimitive;

use crate::color_convert::rgba_to_vec3;

pub trait Coloration<T>: Debug + Send + Sync
where
    T: RealField + ToPrimitive,
{
    fn color(&self, texture_coords: &na::Vector2<T>) -> na::Vector3<T>;
}

#[derive(Debug)]
pub struct Color<T>
where
    T: RealField + ToPrimitive,
{
    pub color: na::Vector3<T>,
}

impl<T> Coloration<T> for Color<T>
where
    T: RealField + ToPrimitive,
{
    fn color(&self, _texture_coords: &na::Vector2<T>) -> na::Vector3<T> {
        self.color
    }
}

fn wrap<T>(val: T, bound: u32) -> u32
where
    T: RealField + ToPrimitive,
{
    let signed_bound = bound as i32;
    let float_coord = val * T::from_i32(signed_bound).unwrap();
    let wrapped_coord = float_coord.to_i32().unwrap() % signed_bound;
    if wrapped_coord < 0 {
        (wrapped_coord + signed_bound) as u32
    } else {
        wrapped_coord as u32
    }
}

#[derive(Debug)]
pub struct Texture {
    pub texture: DynamicImage,
}

impl<T> Coloration<T> for Texture
where
    T: RealField + ToPrimitive,
{
    fn color(&self, texture_coords: &na::Vector2<T>) -> na::Vector3<T> {
        let tex_x = wrap(texture_coords.x, self.texture.width());
        let tex_y = wrap(texture_coords.y, self.texture.height());

        rgba_to_vec3(&self.texture.get_pixel(tex_x, tex_y))
    }
}
