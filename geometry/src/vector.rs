use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{angle::Angle, comparators::is_zero};
#[derive(Debug, Clone)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    /// Creates normalized vector in given angle
    pub fn by_angle(angle: &Angle) -> Vector2D {
        Vector2D {
            x: angle.cos(),
            y: angle.sin(),
        }
    }
    pub fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }
    pub fn normalized(&self) -> Option<Vector2D> {
        let len = self.length();
        if is_zero(len) {
            return None;
        }
        return Some(Vector2D {
            x: self.x / len,
            y: self.y / len,
        });
    }
    pub fn get_angle(&self) -> Angle {
        Angle::atan2(self.x, self.y)
    }
}
// arithmetics
impl Add for Vector2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vector2D {
    type Output = Self;
    fn mul(self, s: f32) -> Self {
        Vector2D {
            x: self.x * s,
            y: self.y * s,
        }
    }
}

impl Mul<Vector2D> for f32 {
    type Output = Vector2D;

    fn mul(self, v: Vector2D) -> Vector2D {
        Vector2D {
            x: v.x * self,
            y: v.y * self,
        }
    }
}

impl Div<f32> for Vector2D {
    type Output = Self;

    fn div(self, s: f32) -> Self {
        Vector2D {
            x: self.x / s,
            y: self.y / s,
        }
    }
}

impl Neg for Vector2D {
    type Output = Vector2D;

    fn neg(self) -> Self::Output {
        Vector2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl PartialEq for Vector2D {
    fn eq(&self, other: &Vector2D) -> bool {
        let diff: Vector2D = self.clone() - other.clone();
        is_zero(diff.length())
    }
}
