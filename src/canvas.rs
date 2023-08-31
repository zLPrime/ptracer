use super::sphere::Sphere;
use super::primitives::{Color, Ray};

pub struct Canvas {
    pub(super) width: usize,
    pub(super) height: usize,
    pub(super) buffer: Vec<u32>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let mut buffer = vec![0_u32; width * height];
        Self {
            height,
            width,
            buffer,
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.buffer[x + y * self.width] = color.into();
    }

    pub fn clear(&mut self) {
        self.buffer.iter_mut().for_each(|v| *v = 0);
    }
}
