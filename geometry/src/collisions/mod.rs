use crate::{
    point::*,
    shapes::{
        Shape2D, Shape2DType,
        affine::{Affine2D, EmbedInAffine2D, TypedAffine2D},
    },
};
use collision_predicate_functions::*;

mod collision_predicate_functions;
#[derive(Debug)]
pub struct CollisionsError;
pub trait Collisions<T> {
    fn check_collision(&self, shape: &T) -> Result<bool, CollisionsError>;
}

impl Collisions<Point2D> for Point2D {
    fn check_collision(&self, point: &Point2D) -> Result<bool, CollisionsError> {
        Ok(check_collision_point_point(self, point))
    }
}

impl<T> Collisions<Point2D> for TypedAffine2D<T>
where
    T: Shape2DType,
{
    fn check_collision(&self, point: &Point2D) -> Result<bool, CollisionsError> {
        let aff: Affine2D = self.clone().into();
        aff.check_collision(point)
    }
}

impl<T> Collisions<Affine2D> for TypedAffine2D<T>
where
    T: Shape2DType,
{
    fn check_collision(&self, aff2: &Affine2D) -> Result<bool, CollisionsError> {
        let aff1: Affine2D = self.clone().into();
        aff1.check_collision(aff2)
    }
}

impl<S, T> Collisions<TypedAffine2D<S>> for TypedAffine2D<T>
where
    T: Shape2DType,
    S: Shape2DType,
{
    fn check_collision(&self, t_aff2: &TypedAffine2D<S>) -> Result<bool, CollisionsError> {
        let aff1: Affine2D = self.clone().into();
        let aff2: Affine2D = t_aff2.clone().into();
        aff1.check_collision(&aff2)
    }
}

impl Collisions<Point2D> for Affine2D {
    fn check_collision(&self, point: &Point2D) -> Result<bool, CollisionsError> {
        match &self.shape {
            Shape2D::Rectangle { rect: r } => Ok(check_collision_rect_point(
                &r.embed_affine(&self.point),
                point,
            )),
            Shape2D::Circle { circle: c } => Ok(check_collision_circle_point(
                &c.embed_affine(&self.point),
                point,
            )),
            Shape2D::Point => Ok(check_collision_point_point(&self.point, point)),
            Shape2D::Line { line: l } => Ok(check_collision_line_point(
                &l.embed_affine(&self.point),
                point,
            )),
            Shape2D::Segment { segment: s } => Ok(check_collision_segment_point(
                &TypedAffine2D::new(self.point, s.clone()),
                point,
            )),
        }
    }
}

impl Collisions<Affine2D> for Point2D {
    fn check_collision(&self, shape: &Affine2D) -> Result<bool, CollisionsError> {
        shape.check_collision(self)
    }
}

impl Collisions<Affine2D> for Affine2D {
    fn check_collision(&self, other: &Affine2D) -> Result<bool, CollisionsError> {
        match &self.shape {
            Shape2D::Point => other.check_collision(&self.point),
            Shape2D::Line { line: l } => match &other.shape {
                Shape2D::Point => Ok(check_collision_line_point(
                    &l.embed_affine(&self.point),
                    &other.point,
                )),
                Shape2D::Line { line: l2 } => Ok(check_collision_line_line(
                    &l.embed_affine(&self.point),
                    &l2.embed_affine(&other.point),
                )),
                Shape2D::Segment { segment: s } => Ok(check_collision_line_segment(
                    &l.embed_affine(&self.point),
                    &s.embed_affine(&other.point),
                )),
                Shape2D::Rectangle { rect: r } => Ok(check_collision_rect_line(
                    &r.embed_affine(&other.point),
                    &l.embed_affine(&self.point),
                )),
                Shape2D::Circle { circle: c } => Ok(check_collision_circle_line(
                    &c.embed_affine(&other.point),
                    &l.embed_affine(&self.point),
                )),
            },
            Shape2D::Segment { segment: s } => match &other.shape {
                Shape2D::Point => Ok(check_collision_segment_point(
                    &s.embed_affine(&self.point),
                    &other.point,
                )),
                Shape2D::Rectangle { rect } => Ok(check_collision_rect_segment(
                    &rect.embed_affine(&other.point),
                    &s.embed_affine(&self.point),
                )),
                Shape2D::Circle { circle } => Ok(check_collision_circle_segment(
                    &circle.embed_affine(&other.point),
                    &s.embed_affine(&self.point),
                )),
                Shape2D::Line { line: l } => Ok(check_collision_line_segment(
                    &l.embed_affine(&other.point),
                    &s.embed_affine(&self.point),
                )),
                Shape2D::Segment { segment: s2 } => Ok(check_collision_segment_segment(
                    &s2.embed_affine(&other.point),
                    &s.embed_affine(&self.point),
                )),
            },
            Shape2D::Circle { circle: c } => match &other.shape {
                Shape2D::Point => Ok(check_collision_circle_point(
                    &c.embed_affine(&self.point),
                    &other.point,
                )),
                Shape2D::Rectangle { rect } => Ok(check_collision_circle_rect(
                    &c.embed_affine(&self.point),
                    &rect.embed_affine(&other.point),
                )),
                Shape2D::Circle { circle } => Ok(check_collision_circle_circle(
                    &c.embed_affine(&self.point),
                    &circle.embed_affine(&other.point),
                )),
                Shape2D::Line { line } => Ok(check_collision_circle_line(
                    &c.embed_affine(&self.point),
                    &line.embed_affine(&other.point),
                )),
                Shape2D::Segment { segment } => Ok(check_collision_circle_segment(
                    &c.embed_affine(&self.point),
                    &segment.embed_affine(&other.point),
                )),
            },
            Shape2D::Rectangle { rect: r } => match &other.shape {
                Shape2D::Point => Ok(check_collision_rect_point(
                    &r.embed_affine(&self.point),
                    &other.point,
                )),
                Shape2D::Rectangle { rect } => Ok(check_collision_rect_rect(
                    &r.embed_affine(&self.point),
                    &rect.embed_affine(&other.point),
                )),
                Shape2D::Circle { circle } => Ok(check_collision_circle_rect(
                    &circle.embed_affine(&other.point),
                    &r.embed_affine(&self.point),
                )),
                Shape2D::Line { line } => Ok(check_collision_rect_line(
                    &r.embed_affine(&self.point),
                    &line.embed_affine(&other.point),
                )),
                Shape2D::Segment { segment } => Ok(check_collision_rect_segment(
                    &r.embed_affine(&self.point),
                    &segment.embed_affine(&other.point),
                )),
            },
        }
    }
}
