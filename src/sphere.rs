use nalgebra::{Point3, RealField, Vector3};

pub struct Sphere<T: RealField> {
    pub center: Point3<T>,
    pub radius: T,
    pub color: Vector3<T>,
}
