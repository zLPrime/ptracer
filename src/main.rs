use minifb::{Key, Window, WindowOptions};

mod primitives;
use primitives::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

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