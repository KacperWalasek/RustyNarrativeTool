use crate::{angle::Angle, vector::Vector2D};

use super::{
    Shape2D, Shape2DType,
    affine::{EmbedInAffine2D, TypedAffine2D},
    segment::Segment2D,
};

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}
impl Shape2DType for Rectangle {}

impl From<Rectangle> for Shape2D {
    fn from(rect: Rectangle) -> Self {
        Shape2D::Rectangle { rect }
    }
}

impl TypedAffine2D<Rectangle> {
    pub fn get_left(&self) -> f32 {
        self.point.x - self.shape.width / 2.0
    }
    pub fn get_right(&self) -> f32 {
        self.point.x + self.shape.width / 2.0
    }
    pub fn get_top(&self) -> f32 {
        self.point.y + self.shape.height / 2.0
    }
    pub fn get_bottom(&self) -> f32 {
        self.point.y - self.shape.height / 2.0
    }

    pub fn get_segments(&self) -> [TypedAffine2D<Segment2D>; 4] {
        let horizontal_seg = Segment2D {
            angle: Angle::degrees(0.0),
            length: self.shape.width,
        };
        let vertical_seg = Segment2D {
            angle: Angle::degrees(90.0),
            length: self.shape.height,
        };
        let x_vec = Vector2D { x: 1.0, y: 0.0 };
        let y_vec = Vector2D { x: 0.0, y: 1.0 };
        [
            horizontal_seg.embed_affine(&(self.point + y_vec.clone() * self.shape.height / 2.0)),
            vertical_seg.embed_affine(&(self.point + x_vec.clone() * self.shape.width / 2.0)),
            horizontal_seg.embed_affine(&(self.point - y_vec.clone() * self.shape.height / 2.0)),
            vertical_seg.embed_affine(&(self.point - x_vec.clone() * self.shape.width / 2.0)),
        ]
    }
}
