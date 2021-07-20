use nalgebra as na;
use num::ToPrimitive;

use crate::sphere::Sphere;

pub struct Intersection<'a, T, U>
where
    T: na::RealField,
    U: na::RealField + ToPrimitive,
{
    pub distance: T,
    pub object: &'a Sphere<T, U>,
}

impl<T, U> Intersection<'_, T, U>
where
    T: na::RealField,
    U: na::RealField + ToPrimitive,
{
	pub fn new(distance: T, object: &Sphere<T, U>) -> Intersection<T, U> {
		Intersection {
			distance,
			object,
		}
	}
}
