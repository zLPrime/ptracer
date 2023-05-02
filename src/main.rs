use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

struct Canvas {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let mut buffer = vec![0_u32; width * height];
        Self { height, width, buffer }
    }
}

impl Canvas {
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        self.buffer[x + y * self.width] = color;
    }
}

fn main() {
    let canvas = init_canvas();

    display_buffer(&canvas.buffer);
}

fn init_canvas() -> Canvas {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let mut c:u32 = 255;
    canvas.draw_pixel(100, 100, c);
    canvas.draw_pixel(300, 300, c);
    canvas.draw_pixel(0, 0, c);
    canvas.draw_pixel(0, 100, c);
    canvas.draw_pixel(300, 400, c);
    canvas
}

fn display_buffer(buffer: &[u32]) {
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
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}