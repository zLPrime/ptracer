use minifb::{Key, Window, WindowOptions};

mod primitives;
mod canvas;
mod camera;
mod scene;
mod sphere;
use primitives::*;
use primitives::vec3d::{Point3d, Vec3d};
use canvas::Canvas;
use camera::Camera;
use scene::Scene;
use sphere::Sphere;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut scene = init_scene();

    display(&mut scene);
}

fn init_canvas() -> Canvas {
    let canvas = Canvas::new(WIDTH, HEIGHT);
    canvas
}

fn init_scene() -> Scene {
    let orig_direction = Vec3d {x: -5., y: 0., z: 0.};
    let direction = orig_direction.rotate_x(0.);
    let camera = Camera { location: Point3d { x: 0., y: 0., z: 0. }, direction };
    let small_sphere = Sphere { center: Point3d { x: -5., y: -0.75, z: 0. }, radius: 0.75, color: Color::new(1., 0.3, 0.3) };
    let small_sphere_2 = Sphere { center: Point3d { x: -5., y: 0.75, z: 0. }, radius: 0.75, color: Color::new(0.3, 1.0, 0.3) };
    let big_sphere = Sphere { center: Point3d { x: -5., y: 0., z: -50. }, radius: 49.25, color: Color::new(0.8, 0.8, 0.8) };
    let light_source = Vec3d::new(1., 0., 0.5).normalize();
    let scene = Scene { spheres: vec![small_sphere
        , small_sphere_2
        , big_sphere
         ], camera, light_source };
    scene
}

fn display(scene: &mut Scene) {
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let r_step = 0.05_f32;
    let m_step = 0.2_f32;
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut canvas = init_canvas();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        scene.camera.render(&mut canvas, &scene);
        if window.is_key_down(Key::Left) {
            scene.camera.rotate_x(r_step);
        } else if (window.is_key_down(Key::Right)) {
            scene.camera.rotate_x(-r_step);
        }

        if window.is_key_down(Key::W) {
            scene.camera.move_forward(m_step);
        } else if (window.is_key_down(Key::S)) {
            scene.camera.move_forward(-m_step);
        }

        if window.is_key_down(Key::A) {
            scene.camera.move_left(m_step);
        } else if (window.is_key_down(Key::D)) {
            scene.camera.move_left(-m_step);
        }
        
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
        .update_with_buffer(&canvas.buffer, canvas.width, canvas.height)
        .unwrap();
    
        canvas.clear();
    }
}