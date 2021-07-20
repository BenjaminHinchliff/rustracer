use nalgebra as na;
use num::ToPrimitive;

use crate::intersectable::Intersectable;

#[derive(Debug)]
pub struct Intersection<'a, T>
where
    T: na::RealField + ToPrimitive,
{
    pub distance: T,
    pub object: &'a dyn Intersectable<T>,
}

impl<T> Intersection<'_, T>
where
    T: na::RealField + ToPrimitive,
{
    pub fn new(distance: T, object: &dyn Intersectable<T>) -> Intersection<T> {
        Intersection { distance, object }
    }
}
