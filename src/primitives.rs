use core::convert::From;
use core::mem::transmute_copy;

#[derive(Copy,Clone)]
pub struct Point3d {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

impl Vec3d {
    pub fn normalize(&self) -> Vec3d {
        let mut norm: Vec3d = self.clone();
        let module = self.x*self.x + self.y*self.y + self.z*self.z;
        let mag = module.sqrt();
        norm.x = norm.x/mag;
        norm.y = norm.y/mag;
        norm.z = norm.z/mag;
        norm
    }
}

pub type Vec3d = Point3d;

pub struct Ray {
    origin: Point3d,
    pub(crate) direction: Vec3d,
}

pub struct Canvas {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) buffer: Vec<u32>,
}

pub struct Camera {
    pub(crate) location: Point3d,
    pub(crate) direction: Vec3d,
}

impl Camera {
    pub fn render(&self, canvas: &mut Canvas) {
        let mut color = Color { blue: 0., green: 0., red: 0. };
        for x in 0..canvas.width {
            color.green = (x as f32 / canvas.width as f32);
            for y in 0..canvas.height {
                color.red = (y as f32 / canvas.height as f32);
                canvas.draw_pixel(x, y, color);
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Color {
    pub(crate) red: f32,
    pub(crate) green: f32,
    pub(crate) blue: f32,
}

struct Color8b {
    blue: u8,
    green: u8,
    red: u8,
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
        };
        unsafe {transmute_copy(&color8b)}
    }
}
