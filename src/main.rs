use std::mem::transmute_copy;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

#[derive(Copy,Clone)]
struct Point3d {
    x: f32,
    y: f32,
    z: f32,
}

type Vec3d = Point3d;

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
struct Color {
    red: f32,
    green: f32,
    blue: f32,
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

fn main() {
    let canvas = init_canvas();

    display_canvas(&canvas);
}

fn get_ray_color(ray: Ray) -> Color {
    let mut color = Color {red: 0., green: 0., blue: 0.};
    let norm_dir = ray.direction.normalize();
    color.red = norm_dir.x;
    color.green = norm_dir.y;
    color.blue = norm_dir.z;
    return color;
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