use std::ops::{Add, Mul};

#[derive(Debug, Clone)]

pub struct Color (pub u8, pub u8, pub u8);

impl Color {
    pub fn new(r: u8, g: u8, b:u8) -> Self {
        Color(r, g, b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, t: f64) -> Color {
        Color ((self.0 as f64 * t).round().min(255.0) as u8, (self.1 as f64 * t).round().min(255.0) as u8, (self.2 as f64 * t).round().min(255.0) as u8)
    }
}


impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        let r = (self.0 as u16 + other.0 as u16).min(255) as u8;
        let g = (self.1 as u16 + other.1 as u16).min(255) as u8;
        let b = (self.2 as u16 + other.2 as u16).min(255) as u8;
        Color (r, g, b)
    }
}