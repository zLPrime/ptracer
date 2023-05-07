use std::mem::transmute_copy;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

struct Vec3d {
    x: f32,
    y: f32,
    z: f32,
}

type Point3d = Vec3d;

struct Ray {
    origin: Point3d,
    direction: Vec3d,
}

struct Canvas {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

struct Camera {
    location: Point3d,
    direction: Vec3d,
}

impl Camera {
    pub fn render(&self, canvas: &mut Canvas) {
        let mut color = Color { blue: 0, green: 0, red: 0 };
        for x in 0..canvas.width {
            color.green = (x * 256 / canvas.width).try_into().unwrap();
            for y in 0..canvas.height {
                color.red = (y * 256 / canvas.height).try_into().unwrap();
                canvas.draw_pixel(x, y, color);
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Color {
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
        unsafe {transmute_copy(&color)}
    }
}

fn main() {
    let canvas = init_canvas();

    display_canvas(&canvas);
}

fn init_canvas() -> Canvas {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let camera = Camera { location: Point3d { x: 0., y: 0., z: 0. }, direction: Vec3d {x: 0., y: 0., z: -1.}};
    camera.render(&mut canvas);
    canvas
}

fn display_canvas(canvas: &Canvas) {
    let buffer = &canvas.buffer;
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(buffer, canvas.width, canvas.height)
            .unwrap();
    }
}