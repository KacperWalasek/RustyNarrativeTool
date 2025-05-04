use core::f32;
use std::f32::consts::PI;

use crate::comparators::are_equal;

#[derive(Debug, Clone)]
pub struct Angle {
    value: f32,
    representation: AngleRepresentation,
}
#[derive(Debug, Clone)]
pub enum AngleRepresentation {
    Radians,
    Degrees,
}

impl Angle {
    pub const fn radians(radians: f32) -> Angle {
        Angle {
            value: radians,
            representation: AngleRepresentation::Radians,
        }
    }

    pub const fn degrees(degrees: f32) -> Angle {
        Angle {
            value: degrees,
            representation: AngleRepresentation::Degrees,
        }
    }

    pub const fn as_radians(&self) -> f32 {
        match self.representation {
            AngleRepresentation::Radians => self.value,
            AngleRepresentation::Degrees => self.value * PI / 180.0,
        }
    }

    pub const fn as_degrees(&self) -> f32 {
        match self.representation {
            AngleRepresentation::Radians => self.value * 180.0 / PI,
            AngleRepresentation::Degrees => self.value,
        }
    }

    pub fn sin(&self) -> f32 {
        f32::sin(self.as_radians())
    }

    pub fn cos(&self) -> f32 {
        f32::cos(self.as_radians())
    }

    pub fn tan(&self) -> f32 {
        f32::tan(self.as_radians())
    }

    pub fn atan2(x: f32, y: f32) -> Angle {
        Angle::radians(y.atan2(x))
    }

    pub fn normalized(&self) -> Angle {
        let mut degrees = self.as_degrees() % 360.0;
        if degrees < 0.0 {
            degrees += 360.0;
        }
        Angle::degrees(degrees)
    }
    pub fn is_inline_with(&self, angle: &Angle) -> bool {
        let normed1 = self.normalized();
        let normed2 = angle.normalized();
        are_equal(normed1.as_degrees() % 180.0, normed2.as_degrees() % 180.0)
    }
}
