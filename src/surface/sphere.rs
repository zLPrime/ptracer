use crate::primitives::vec3d::{Point3d, Vec3d};
use crate::primitives::Ray;

use super::material::Material;
use super::Surface;

#[derive(Debug)]
pub struct Sphere {
    pub center: Point3d,
    pub radius: f32,
    pub material: Material,
}

impl Surface for Sphere {
    fn get_normal(&self, point: Point3d) -> Vec3d {
        return (point - self.center) / self.radius;
    }

    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let oc = ray.origin - self.center;
    
        let k1 = ray.direction * ray.direction;
        let k2 = oc * ray.direction * 2.;
        let k3 = oc * oc - self.radius * self.radius;
    
        let discr = k2 * k2 - 4. * k1 * k3;
        if discr < 0. {
            return None
        }
    
        let discr_sqrt = discr.sqrt();

        let t1 = (-k2 + discr_sqrt) / (2. * k1);
        let t2 = (-k2 - discr_sqrt) / (2. * k1);
    
        //TODO could do this check earlier to optimize
        if t1 < 0.001 && t2 < 0.001 {
            return None
        }
    
        return Some(f32::min(t1, t2))
    }
}
