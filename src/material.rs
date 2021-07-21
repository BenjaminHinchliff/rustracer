use crate::coloration::Coloration;

#[derive(Debug)]
pub struct Material<T> {
    pub color: Box<dyn Coloration<T>>,
    pub albedo: T,
}
