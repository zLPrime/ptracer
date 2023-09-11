use crate::primitives::vec3d::Vec3d;

use super::primitives::{Color, Ray};

use super::camera::Camera;
use super::sphere::Sphere;

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub camera: Camera,
    pub light_source: Vec3d,
}

fn get_background_color(ray: &Ray) -> Color {
    let norm_dir = ray.direction.normalize();
    let color = Color::new(norm_dir.x, norm_dir.y, norm_dir.z);
    return color;
}

fn get_lightness(ray: &Ray, scene: &Scene) -> Color {
    for sphere in &scene.spheres {
        let intersection = sphere.intersect(ray);
        match intersection {
            Some(_) => return Color::new(0., 0., 0.),
            None => {}
        }
    }
    let norm_dir = ray.direction.normalize();
    let lightness = f32::max(0., norm_dir * scene.light_source);
    let color = Color::new(lightness, lightness, lightness);
    return color;
}

//TODO move it to camera?
pub fn get_ray_color(ray: &Ray, scene: &Scene, depth: u8) -> Color {
    if depth > 0 {
        let mut final_value = f32::MAX;
        let mut final_sphere = None;
        for sphere in &scene.spheres {
            let intersection = sphere.intersect(&ray);
            match intersection {
                Some(new_value) => {
                    final_value = f32::min(final_value, new_value);
                    if final_value == new_value {
                        final_sphere = Some(sphere);
                    }
                }
                None => continue,
            }
        }
        match final_sphere {
            Some(sphere) => {
                let hit_point = ray.origin + ray.direction * final_value;
                let normal = sphere.get_normal(hit_point);
                let bounce_direction =
                    if depth > 1 {
                        let mut bd = Vec3d::random();
                        if normal * bd < 0. {
                            bd = bd * -1.;
                        }
                        bd
                    }
                    else {
                        scene.light_source
                    };
                let bounce_ray = Ray { origin: hit_point, direction: bounce_direction };
                return sphere.color * get_ray_color(&bounce_ray, scene, depth - 1) * (bounce_direction * normal)
            },
            None => {},
        }
    }
    // return get_background_color(ray);
    return get_lightness(ray, &scene)
}
