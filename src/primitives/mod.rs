use core::convert::From;
use std::mem::transmute_copy;
use vec3d::{Point3d, Vec3d};
use matrix::{Matrix3x3, Matrix3x1};

pub mod vec3d;
mod matrix;

#[derive(Debug)]
struct Ray {
    origin: Point3d,
    direction: Vec3d,
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Point3d,
    pub radius: f32,
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

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub camera: Camera,
}

impl Camera {
    pub fn render(&self, canvas: &mut Canvas, scene: &Scene) {
        let ratio = canvas.width as f32 / canvas.height as f32;
        let left = self.get_left().normalize() * ratio;
        let top = self.direction.cross(&left).normalize();
        let top_left = self.direction + top + left;
        let step_v = 2. / (canvas.width - 1) as f32;
        let step_h = 2. / (canvas.height - 1) as f32;
        for y in 0..canvas.height {
            let current_v = top_left - (top * (step_h * (y) as f32));
            for x in 0..canvas.width {
                let direction = current_v - (left * (step_v * (x) as f32));
                let ray = Ray { origin: self.location, direction };
                let intersect = intersect_ray_spheres(ray, &scene.spheres);
                match intersect {
                    Some(_) => canvas.draw_pixel(x, y, Color { red: 1., green: 1., blue: 1. }),
                    None => {}
                }
            }
        }
    }

    pub fn rotate_x(&mut self, theta: f32) {
        self.direction = self.direction.rotate_x(theta)
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

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.buffer[x + y * self.width] = color.into();
    }

    pub fn clear(&mut self) {
        self.buffer.iter_mut().for_each(|v| *v = 0);
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

fn get_ray_color(ray: Ray) -> Color {
    let mut color = Color {red: 0., green: 0., blue: 0.};
    let norm_dir = ray.direction.normalize();
    color.red = norm_dir.x;
    color.green = norm_dir.y;
    color.blue = norm_dir.z;
    return color;
}

fn intersect_ray_spheres(ray: Ray, spheres: &Vec<Sphere>) -> Option<(f32, f32)> {
    for sphere in spheres {
        let intersection = intersect_ray_sphere(&ray, &sphere);
        match intersection {
            Some(value) => return Some(value),
            None => continue,
        }
    }
    None
}

fn intersect_ray_sphere(ray: &Ray, sphere: &Sphere) -> Option<(f32, f32)> {
    let oc = ray.origin - sphere.center;

    let k1 = ray.direction * ray.direction;
    let k2 = oc * ray.direction * 2.;
    let k3 = oc * oc - sphere.radius * sphere.radius;

    let discr = k2 * k2 - 4. * k1 * k3;
    if (discr < 0.){
        return None
    }

    let t1 = (-k2 + discr.sqrt()) / (2. * k1);
    let t2 = (-k2 - discr.sqrt()) / (2. * k1);
    return Some((t1, t2))
}
