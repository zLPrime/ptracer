use crate::primitives::vec3d::Vec3d;

use super::primitives::{Color, Ray};

use super::camera::Camera;
use super::sphere::Sphere;

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub camera: Camera,
}

fn get_background_color(ray: &Ray) -> Color {
    let mut color = Color {
        red: 0.,
        green: 0.,
        blue: 0.,
    };
    let norm_dir = ray.direction.normalize();
    color.red = norm_dir.x;
    color.green = norm_dir.y;
    color.blue = norm_dir.z;
    return color;
}

//TODO move it to camera?
pub fn get_ray_color(ray: &Ray, spheres: &Vec<Sphere>, depth: u8) -> Color {
    if depth > 0 {
    for sphere in spheres {
        let intersection = sphere.intersect(&ray);
        match intersection {
            Some((value1, value2)) => {
                let value = f32::min(value1, value2);
                let hit_point = ray.origin + ray.direction * value;
                let normal = sphere.get_normal(hit_point);
                let mut bounce_direction = Vec3d::random();
                if normal * bounce_direction < 0. {
                    bounce_direction = bounce_direction * -1.;
                }
                let bounce_ray = Ray { origin: hit_point, direction: bounce_direction };
                return Color {red: 0.8, green: 0.8, blue: 0.8} * get_ray_color(&bounce_ray, spheres, depth - 1)
            }
            None => continue,
        }
    }}
    return get_background_color(ray);
}
