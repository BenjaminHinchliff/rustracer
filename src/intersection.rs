use nalgebra as na;
use num::ToPrimitive;

use crate::intersectable::Intersectable;

pub struct Intersection<'a, T, U>
where
    T: na::RealField,
    U: na::RealField + ToPrimitive,
{
    pub distance: T,
    pub object: &'a dyn Intersectable<T, U>,
}

impl<T, U> Intersection<'_, T, U>
where
    T: na::RealField,
    U: na::RealField + ToPrimitive,
{
    pub fn new(distance: T, object: &dyn Intersectable<T, U>) -> Intersection<T, U> {
        Intersection { distance, object }
    }
}
