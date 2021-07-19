use nalgebra as na;

pub struct Sphere<T: na::RealField> {
    pub center: na::Point3<T>,
    pub radius: T,
    pub color: na::Vector3<T>,
}
