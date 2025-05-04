use super::{
    Shape2D, Shape2DType,
    affine::{EmbedInAffine2D, TypedAffine2D},
    line::Line2D,
};
use crate::{angle::Angle, point::Point2D, vector::Vector2D};

#[derive(Debug, Clone)]
pub struct Segment2D {
    pub angle: Angle,
    pub length: f32,
}
impl Shape2DType for Segment2D {}

impl From<Segment2D> for Shape2D {
    fn from(segment: Segment2D) -> Self {
        Shape2D::Segment { segment }
    }
}

impl Segment2D {
    pub fn get_line(&self) -> Line2D {
        Line2D {
            angle: self.angle.clone(),
        }
    }
}

impl TypedAffine2D<Segment2D> {
    pub fn get_line(&self) -> TypedAffine2D<Line2D> {
        self.shape.get_line().embed_affine(&self.point)
    }

    pub fn is_inline_with(&self, other: &TypedAffine2D<Segment2D>) -> bool {
        let self_angle = &self.shape.angle;
        let point_vec = self.point - other.point;
        self_angle.is_inline_with(&other.shape.angle)
            && self_angle.is_inline_with(&point_vec.get_angle())
    }

    pub fn get_end_points(&self) -> [Point2D; 2] {
        let v = Vector2D::by_angle(&self.shape.angle) * self.shape.length / 2.0;
        [self.point + v.clone(), self.point - v.clone()]
    }
}
