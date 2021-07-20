use nalgebra as na;

use crate::ray::Ray;

pub trait Intersectable<T: na::RealField> {
	fn intersect(&self, ray: &Ray<T>) -> bool;
}