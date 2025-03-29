use super::AnyShape;

pub struct Affine {
    pub shape: Box<dyn AnyShape>,
    pub center: Point,
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Affine {
    pub fn new(center: Point, shape: Box<dyn AnyShape>) -> Affine {
        Affine { shape, center }
    }
}