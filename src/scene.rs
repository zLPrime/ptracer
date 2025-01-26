use crate::primitives::vec3d::Vec3d;
use crate::surface::material::MaterialKind;
use crate::primitives::{Color, Ray};
use crate::surface::mesh::Mesh;
use crate::surface::sphere::Sphere;
use crate::camera::Camera;
use crate::surface::triangle::Triangle;
use crate::surface::Surface;

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub triangles: Vec<Triangle>,
    pub meshes: Vec<Mesh>,
    pub camera: Camera,
    pub light_source: Vec3d,
}

fn get_background_color(ray: &Ray) -> Color {
    let norm_dir = ray.direction.normalize();
    let color = Color::new(norm_dir.x, norm_dir.y, norm_dir.z);
    return color;
}

fn get_lightness(ray: &Ray, scene: &Scene) -> Color {
    if get_any_intersection(ray, &scene.spheres)
    || get_any_intersection(ray, &scene.triangles) {
        return Color::new(0., 0., 0.)
    }
    let norm_dir = ray.direction.normalize();
    let lightness = f32::max(0., norm_dir * scene.light_source);
    let color = Color::new(lightness, lightness, lightness);
    return color;
}

fn get_any_intersection<T: Surface>(ray: &Ray, surfaces: &[T]) -> bool {
    for surface in surfaces {
        let intersection = surface.intersect(ray);
        match intersection {
            Some(_) => return true,
            None => {}
        }
    }
    return false
}

fn get_closest_ditance<'a, 'b, T: Surface>(ray: &'a Ray, surfaces: &'b [T]) -> (f32, Option<&'b T>) {
    let mut closest_surface  = None;
    let mut closest_distance = f32::MAX;
    for surface in surfaces {
        let distance = surface.intersect(ray);
        match distance {
            Some(new_distance) => {
                closest_distance = f32::min(closest_distance, new_distance);
                if closest_distance == new_distance {
                    closest_surface = Some(surface);
                }
            }
            None => continue
        }
    }
    return (closest_distance, closest_surface)
}

//TODO move it to camera?
pub fn get_ray_color(ray: &Ray, scene: &Scene, depth: u8) -> Color {
    if depth > 0 {
        let (dist_to_sphere, sphere) = get_closest_ditance(ray, &scene.spheres);
        let (dist_to_triangle, triangle) = get_closest_ditance(ray, &scene.triangles);

        if dist_to_sphere < dist_to_triangle {
            match sphere {
                Some(surface) => {
                    let (material, bounce_ray) = reflect(ray, dist_to_sphere, surface);
                    return material.color * get_ray_color(&bounce_ray, scene, depth - 1)
                },
                None => {},
            }
        } else {
            match triangle {
                Some(surface) => {
                    let (material, bounce_ray) = reflect(ray, dist_to_triangle, surface);
                    return material.color * get_ray_color(&bounce_ray, scene, depth - 1)
                },
                None => {},
            }
        }
    }
    // return get_background_color(ray);
    return get_lightness(ray, &scene)
}

fn reflect<T: Surface>(ray: &Ray, dist_to_surface: f32, surface: &T) -> (crate::Material, Ray) {
    let hit_point = ray.origin + ray.direction * dist_to_surface;
    let normal = surface.get_normal(hit_point);
    let material = surface.get_material();
    let bounce_direction =
        get_bounce_direction(ray.direction, normal, material.material_kind);
    let bounce_ray = Ray { origin: hit_point, direction: bounce_direction };
    (material, bounce_ray)
}


// w = v - 2 * (v ∙ n) * n
// TODO: encapsulate in material?
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