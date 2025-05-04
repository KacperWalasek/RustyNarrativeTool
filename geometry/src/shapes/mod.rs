use circle::Circle;
use line::Line2D;
use rectangle::Rectangle;
use segment::Segment2D;

pub mod affine;
pub mod circle;
pub mod line;
pub mod rectangle;
pub mod segment;

#[derive(Debug, Clone)]
pub enum Shape2D {
    Rectangle { rect: Rectangle },
    Circle { circle: Circle },
    Line { line: Line2D },
    Segment { segment: Segment2D },
    Point,
}

pub trait Shape2DType: Into<Shape2D> + Clone {}
