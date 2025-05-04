use super::{Shape2D, Shape2DType};

#[derive(Debug, Clone)]
pub struct Circle {
    pub radius: f32,
}
impl Shape2DType for Circle {}

impl From<Circle> for Shape2D {
    fn from(circle: Circle) -> Self {
        Shape2D::Circle { circle }
    }
}
