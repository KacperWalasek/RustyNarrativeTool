use std::ops::{Add, Sub};

use crate::{comparators::is_zero, vector::Vector2D};

#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point2D {
    pub fn zero() -> Point2D {
        Point2D { x: 0.0, y: 0.0 }
    }
}

impl Add<Vector2D> for Point2D {
    type Output = Point2D;

    fn add(self, other: Vector2D) -> Self::Output {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Vector2D> for Point2D {
    type Output = Point2D;

    fn sub(self, other: Vector2D) -> Self::Output {
        Point2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<Point2D> for Point2D {
    type Output = Vector2D;

    fn sub(self, rhs: Point2D) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl PartialEq for Point2D {
    fn eq(&self, other: &Point2D) -> bool {
        let diff: Vector2D = self.clone() - other.clone();
        is_zero(diff.length())
    }
}

pub fn dist2(p1: &Point2D, p2: &Point2D) -> f32 {
    f32::powi(p1.x - p2.x, 2) + f32::powi(p1.y - p2.y, 2)
}

pub fn dist(p1: &Point2D, p2: &Point2D) -> f32 {
    f32::sqrt(dist2(p1, p2))
}
