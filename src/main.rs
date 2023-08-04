use minifb::{Key, Window, WindowOptions};

mod primitives;
use primitives::*;
use primitives::vec3d::{Point3d, Vec3d};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let canvas = init_canvas();

    display_canvas(&canvas);
}

fn init_canvas() -> Canvas {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let orig_direction = Vec3d {x: -5., y: 0., z: 0.};
    let direction = orig_direction.rotate_x(0.);
    let camera = Camera { location: Point3d { x: 0., y: 0., z: 0. }, direction };
    let sphere = Sphere { center: Point3d { x: -5., y: 0., z: 0. }, radius: 0.75};
    let scene = Scene { spheres: vec![sphere]};
    
    camera.render(&mut canvas, &scene);
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