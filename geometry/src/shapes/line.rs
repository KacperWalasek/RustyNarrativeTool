use super::{
    Shape2D, Shape2DType,
    affine::{EmbedInAffine2D, TypedAffine2D},
};
use crate::{
    angle::Angle,
    point::{Point2D, dist2},
};

#[derive(Debug, Clone)]
pub struct Line2D {
    pub angle: Angle,
}
impl Shape2DType for Line2D {}
impl From<Line2D> for Shape2D {
    fn from(line: Line2D) -> Self {
        Shape2D::Line { line }
    }
}

impl Line2D {
    pub fn get_perpendicular(&self) -> Line2D {
        Line2D {
            angle: Angle::degrees(self.angle.as_degrees() + 90.0).normalized(),
        }
    }
}

impl TypedAffine2D<Line2D> {
    fn get_intersection_point_with_vertical(l: &TypedAffine2D<Line2D>, x: f32) -> Point2D {
        let a = l.shape.angle.tan();
        let b = l.point.y - a * l.point.x;
        Point2D { x, y: a * x + b }
    }
    pub fn get_intersection_point(&self, l: &TypedAffine2D<Line2D>) -> Option<Point2D> {
        if self.shape.angle.is_inline_with(&l.shape.angle) {
            return None;
        }
        if self.shape.angle.is_inline_with(&Angle::degrees(90.0)) {
            return Some(Self::get_intersection_point_with_vertical(l, self.point.x));
        }
        if l.shape.angle.is_inline_with(&Angle::degrees(90.0)) {
            return Some(Self::get_intersection_point_with_vertical(self, l.point.x));
        }
        let (tan1, tan2) = (self.shape.angle.tan(), l.shape.angle.tan());
        let b1 = self.point.y - tan1 * self.point.x;
        let b2 = l.point.y - tan2 * l.point.x;
        let x = (b2 - b1) / (tan1 - tan2);
        let y = tan1 * x + b1;
        Some(Point2D { x, y })
    }

    pub fn dist2(&self, point: &Point2D) -> f32 {
        let perp_line = self.shape.get_perpendicular().embed_affine(point);
        let int_point = self
            .get_intersection_point(&perp_line)
            .expect("perpendicular lines should intesect");
        dist2(&int_point, point)
    }
}
