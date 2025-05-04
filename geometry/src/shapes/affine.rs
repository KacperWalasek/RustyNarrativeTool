use crate::point::Point2D;

use super::{Shape2D, Shape2DType};

#[derive(Debug, Clone)]
pub struct Affine2D {
    pub shape: Shape2D,
    pub point: Point2D,
}
impl Affine2D {
    pub const fn new(x: f32, y: f32, shape: Shape2D) -> Self {
        Affine2D {
            point: Point2D { x, y },
            shape,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypedAffine2D<T>
where
    T: Shape2DType,
{
    pub point: Point2D,
    pub shape: T,
}

impl<T> TypedAffine2D<T>
where
    T: Shape2DType,
{
    pub fn new(point: Point2D, shape: T) -> TypedAffine2D<T> {
        TypedAffine2D { point, shape }
    }
}

impl<T> From<TypedAffine2D<T>> for Affine2D
where
    T: Shape2DType,
{
    fn from(typed_affine: TypedAffine2D<T>) -> Self {
        Affine2D {
            point: typed_affine.point,
            shape: typed_affine.shape.into(),
        }
    }
}

pub trait EmbedInAffine2D {
    type AffineType;
    fn embed_affine(&self, p: &Point2D) -> Self::AffineType;
}

impl<T> EmbedInAffine2D for T
where
    T: Shape2DType,
{
    type AffineType = TypedAffine2D<T>;

    fn embed_affine(&self, p: &Point2D) -> Self::AffineType {
        TypedAffine2D::new(*p, self.clone())
    }
}

impl EmbedInAffine2D for Shape2D {
    type AffineType = Affine2D;

    fn embed_affine(&self, p: &Point2D) -> Self::AffineType {
        Affine2D::new(p.x, p.y, self.clone())
    }
}
