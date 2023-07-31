use core::convert::From;
use std::mem::transmute_copy;
use vec3d::{Point3d, Vec3d};

use crate::get_ray_color;

pub mod vec3d;

pub struct Ray {
    origin: Point3d,
    pub(super) direction: Vec3d,
}

pub struct Canvas {
    pub(super) width: usize,
    pub(super) height: usize,
    pub(super) buffer: Vec<u32>,
}

pub struct Camera {
    pub location: Point3d,
    pub direction: Vec3d,
}

impl Camera {
    pub fn render(&self, canvas: &mut Canvas) {
        let ratio = canvas.width as f32 / canvas.height as f32;
        let left = self.get_left().normalize() * ratio;
        let top = self.direction.cross(&left).normalize();
        let top_left = self.direction + top + left;
        let step_v = 2. / canvas.width as f32;
        let step_h = 2. / canvas.height as f32;
        for y in 0..canvas.height {
            let current_v = top_left - (top * (step_v * (y as f32)));
            for x in 0..canvas.width {
                let direction = current_v - (left * (step_h * x as f32));
                let ray = Ray { origin: self.location, direction };
                canvas.draw_pixel(x, y, get_ray_color(ray));
            }
        }
    }
    
    fn get_left(&self) -> Vec3d {
        let top = Vec3d::new(0.,0.,1.);
        top.cross(&self.direction)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub(super) red: f32,
    pub(super) green: f32,
    pub(super) blue: f32,
}

struct Color8b {
    blue: u8,
    green: u8,
    red: u8,
    alpha: u8,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let mut buffer = vec![0_u32; width * height];
        Self { height, width, buffer }
    }
}

impl Canvas {
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.buffer[x + y * self.width] = color.into();
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        let color8b = Color8b {
            red:   (color.red   * 256.) as u8,
            green: (color.green * 256.) as u8,
            blue:  (color.blue  * 256.) as u8,
            alpha: 0_u8,
        };
        unsafe {transmute_copy(&color8b)}
    }
}
