use crate::primitives::vec3d::Vec3d;
use crate::sphere::MaterialKind;

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
                    get_bounce_direction(ray.direction, normal, sphere.material.material_kind);
                let bounce_ray = Ray { origin: hit_point, direction: bounce_direction };
                return sphere.material.color * get_ray_color(&bounce_ray, scene, depth - 1) * (bounce_direction * normal)
            },
            None => {},
        }
    }
    // return get_background_color(ray);
    return get_lightness(ray, &scene)
}


// w = v - 2 * (v ∙ n) * n
fn get_bounce_direction(ray_direction: Vec3d, normal: Vec3d, material_kind: MaterialKind) -> Vec3d {
    match material_kind {
        MaterialKind::Diffuse => {
            let mut bd = Vec3d::random_unit();
            if normal * bd < 0. {
                bd = bd * -1.;
            }
            bd + 0.5 * normal
        },
        MaterialKind::Glossy => {
            let n = normal;
            let v = ray_direction;
            let w = v - 2. * v.dot(&n) * n;
            w
        }
    }

}