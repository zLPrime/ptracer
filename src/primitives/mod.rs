use vec3d::{Point3d, Vec3d};
use std::ops;

pub mod vec3d;
pub mod rand;
mod matrix;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3d,
    pub direction: Vec3d,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub(super) red: f32,
    pub(super) green: f32,
    pub(super) blue: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color { red, green, blue }
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue
        }
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs
        }
    }
}

// TODO remove it
impl ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue
        }
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        // Pack into 0x00RRGGBB as expected by minifb.
        let red = (color.red * 256.) as u8 as u32;
        let green = (color.green * 256.) as u8 as u32;
        let blue = (color.blue * 256.) as u8 as u32;
        (red << 16) | (green << 8) | blue
    }
}
