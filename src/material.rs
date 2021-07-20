use nalgebra as na;

#[derive(Debug)]
pub struct Material<T> {
    pub color: na::Vector3<T>,
    pub albedo: T,
}
