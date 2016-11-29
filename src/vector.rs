use std::ops::{Add, Mul, Div};
use std::iter::{Iterator, Sum};

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    magnitude: f32,
    radians: f32,
}

impl Vector {
    pub fn new(magnitude: f32, radians: f32) -> Self {
        Vector {
            magnitude: magnitude,
            radians: radians,
        }
    }

    pub fn new_from_angle(magnitude: f32, angle: f32) -> Self {
        Self::new(magnitude, angle.to_radians())
    }
    // fn get_horizontal_component(&self) -> f32 {
    // self.radians.cos() * self.magnitude
    // }
    // fn get_vertical_component(&self) -> f32 {
    // self.radians.sin() * self.magnitude
    // }

    // vertical, horizontal
    pub fn get_components(&self) -> (f32, f32) {
        let (sin, cos) = self.radians.sin_cos();
        (sin * self.magnitude, cos * self.magnitude)
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude
    }

    pub fn radians(&self) -> f32 {
        self.radians
    }
}

impl Default for Vector {
    fn default() -> Self {
        Vector {
            magnitude: 0.,
            radians: 0.,
        }
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        fn pythagorean(a: f32, b: f32) -> f32 {
            (a.powi(2) + b.powi(2)).sqrt()
        }

        let self_c = self.get_components();
        let other_c = other.get_components();
        let combined_c = (self_c.0 + other_c.0, self_c.1 + other_c.1);

        Vector {
            magnitude: pythagorean(combined_c.0, combined_c.1),
            radians: combined_c.0.atan2(combined_c.1),
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Vector { magnitude: self.magnitude * scalar, ..self }
    }
}

impl Div<f32> for Vector {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Vector { magnitude: self.magnitude / scalar, ..self }
    }
}

impl Sum for Vector {
    fn sum<I>(iter: I) -> Self
        where I: Iterator<Item = Self>
    {
        iter.fold(Self::default(), |sum, v| sum + v)
    }
}
