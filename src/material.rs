use crate::coloration::Coloration;

#[derive(Debug, Clone, PartialEq)]
pub enum SurfaceType<T> {
    Diffuse,
    Reflective { reflectivity: T },
}

#[derive(Debug)]
pub struct Material<T> {
    pub color: Box<dyn Coloration<T>>,
    pub surface: SurfaceType<T>,
    pub albedo: T,
}
