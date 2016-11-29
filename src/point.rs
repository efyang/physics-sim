use std::ops::Add;
use super::vector::Vector;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x: x, y: y }
    }
    pub fn distance_to(&self, other: &Point) -> f32 {
        self.distance_to_squared(other).sqrt()
    }
    pub fn distance_to_squared(&self, other: &Point) -> f32 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, other: Vector) -> Self {
        let (yc, xc) = other.get_components();
        Point {
            x: self.x + xc,
            y: self.y + yc,
        }
    }
}
