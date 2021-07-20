use std::fmt::Debug;

use nalgebra as na;
use num::ToPrimitive;

pub trait Light<T>: Debug
where
    T: na::RealField + ToPrimitive,
{
    fn color(&self) -> na::Vector3<T>;
    fn direction_from(&self, hit_point: &na::Point3<T>) -> na::Vector3<T>;
    fn intensity(&self, hit_point: &na::Point3<T>) -> T;
    fn distance(&self, hit_point: &na::Point3<T>) -> T;
}
