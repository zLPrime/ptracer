use crate::primitives::Color;

use super::primitives::vec3d::{Point3d, Vec3d};
use super::primitives::Ray;

#[derive(Debug)]
pub struct Sphere {
    pub center: Point3d,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn get_normal(&self, point: Point3d) -> Vec3d {
        return (point - self.center) / self.radius;
    }

    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        let oc = ray.origin - self.center;
    
        let k1 = ray.direction * ray.direction;
        let k2 = oc * ray.direction * 2.;
        let k3 = oc * oc - self.radius * self.radius;
    
        let discr = k2 * k2 - 4. * k1 * k3;
        if discr < 0. {
            return None
        }
    
        let t1 = (-k2 + discr.sqrt()) / (2. * k1);
        let t2 = (-k2 - discr.sqrt()) / (2. * k1);
    
        //TODO could do this check earlier to optimize
        if t1 < 0.001 && t2 < 0.001 {
            return None
        }
    
        return Some(f32::min(t1, t2))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MaterialKind {
    Glossy,
    Diffuse,
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub material_kind: MaterialKind,
    pub color: Color,
}
