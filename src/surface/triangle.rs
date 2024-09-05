use crate::primitives::vec3d::{Point3d, Vec3d};
use crate::primitives::Ray;

use super::material::Material;
use super::Surface;

#[derive(Debug)]
pub struct Triangle {
    a: Point3d,
    b: Point3d,
    c: Point3d,
    material: Material,
}

impl Triangle {
    pub fn new(a: Point3d, b: Point3d, c: Point3d, material: Material) -> Self {
        Self {a, b, c, material}
    }
}

impl Surface for Triangle {
    fn get_normal(&self, point: Point3d) -> Vec3d {
        let a_b = self.b - self.a;
        let b_c = self.c - self.b;
        return a_b.cross(&b_c)
    }

    fn intersect(&self, ray: &Ray) -> Option<f32> {
        moller_trumbore_intersection(ray, &self)
    }
    
    fn get_material(&self) -> Material {
        return self.material
    }
}

fn moller_trumbore_intersection (ray: &Ray, triangle: &Triangle) -> Option<f32> {
    let e1 = triangle.b - triangle.a;
    let e2 = triangle.c - triangle.a;

    let ray_cross_e2 = ray.direction.cross(&e2);
    let det = e1.dot(&ray_cross_e2);

    if det < f32::EPSILON {
        return None; // This ray does not face the triangle.
    }

    let inv_det = 1.0 / det;
    let s = ray.origin - triangle.a;
    let u = inv_det * s.dot(&ray_cross_e2);
    if u < 0.0 || u > 1.0 {
        return None;
    }

    let s_cross_e1 = s.cross(&e1);
    let v = inv_det * ray.direction.dot(&s_cross_e1);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    // At this stage we can compute t to find out where the intersection point is on the line.
    let t = inv_det * e2.dot(&s_cross_e1);

    if t > f32::EPSILON { // ray intersection
        return Some(t)
    }
    else
    {
        return None
    }
}