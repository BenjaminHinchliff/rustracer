use std::fmt::Debug;

use nalgebra as na;
use num::ToPrimitive;

use crate::{Material, ray::Ray};

pub trait Intersectable<T>: Debug + Sync + Send
where
    T: na::RealField + ToPrimitive,
{
    fn intersect(&self, ray: &Ray<T>) -> Option<T>;
    fn surface_normal(&self, hit_point: &na::Point3<T>) -> na::Vector3<T>;
    fn texture_coords(&self, hit_point: &na::Point3<T>) -> na::Vector2<T>;
    fn material(&self) -> &Material<T>;
}


#[derive(Debug)]
pub struct Sphere<T>
where
    T: na::RealField + ToPrimitive,
{
    pub center: na::Point3<T>,
    pub radius: T,
    pub material: Material<T>,
}

impl<T> Intersectable<T> for Sphere<T>
where
    T: na::RealField + ToPrimitive,
{
    fn intersect(&self, ray: &Ray<T>) -> Option<T> {
        let l = self.center - ray.origin;
        let adj = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj * adj);
        let r2 = self.radius * self.radius;
        if d2 > r2 {
            return None;
        }

        let thc = (r2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < T::zero() && t1 < T::zero() {
            return None;
        } else if t0 < T::zero() {
            Some(t1)
        } else if t1 < T::zero() {
            Some(t0)
        } else {
            let distance = t0.min(t1);
            Some(distance)
        }
    }

    fn surface_normal(&self, hit_point: &na::Point3<T>) -> na::Vector3<T> {
        (*hit_point - self.center).normalize()
    }

    fn texture_coords(&self, hit_point: &na::Point3<T>) -> na::Vector2<T> {
        let hit_vec = hit_point - self.center;
        let x = T::one() + hit_vec.z.atan2(hit_vec.x) / T::pi() * T::from_f64(0.5).unwrap();
        let y = (hit_vec.y / self.radius).acos() / T::pi();
        na::Vector2::new(x, y)
    }

    fn material(&self) -> &Material<T> {
        &self.material
    }
}

#[derive(Debug)]
pub struct Plane<T>
where
    T: na::RealField + ToPrimitive,
{
    pub origin: na::Point3<T>,
    pub normal: na::Vector3<T>,
    pub material: Material<T>,
}

impl<T> Intersectable<T> for Plane<T>
where
    T: na::RealField + ToPrimitive,
{
    fn intersect(&self, ray: &Ray<T>) -> Option<T> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > na::convert(1e-6) {
            let v = self.origin - ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= T::zero() {
                return Some(distance);
            }
        }
        None
    }

    fn surface_normal(&self, _: &na::Point3<T>) -> na::Vector3<T> {
        -self.normal
    }

    fn texture_coords(&self, hit_point: &na::Point3<T>) -> na::Vector2<T> {
        let mut x_axis = self
            .normal
            .cross(&na::Vector3::new(T::zero(), T::zero(), T::one()));
        if x_axis.norm() == T::zero() {
            x_axis = self
                .normal
                .cross(&na::Vector3::new(T::zero(), T::one(), T::zero()));
        }

        let y_axis = self.normal.cross(&x_axis);

        let hit_vec = hit_point - self.origin;
        na::Vector2::new(hit_vec.dot(&x_axis), hit_vec.dot(&y_axis))
    }

    fn material(&self) -> &Material<T> {
        &self.material
    }
}

