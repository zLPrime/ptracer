//use image::{DynamicImage, ImageBuffer, Rgb};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let buffer: Vec<u32> = init_buffer();

    display_buffer(&buffer);
}

fn init_buffer() -> Vec<u32> {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut c:u32 = 0;
    for i in buffer.iter_mut() {
        c += 1;
        *i = c; // write something more funny here!
    }
    buffer
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