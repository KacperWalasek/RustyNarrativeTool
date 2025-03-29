use super::Shape;

pub struct Rectangle {
    pub width: f32,
    pub heigth: f32,
}

pub struct Circle {
    pub radius: f32,
}

pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Shape for Rectangle {}
impl Shape for Circle {}
impl Shape for Vector {}