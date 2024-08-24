use crate::primitives::vec3d::{Point3d, Vec3d};
use crate::primitives::Ray;

use super::material::Material;
use super::Surface;

#[derive(Debug)]
pub struct Triangle {
    pub center: Point3d,
    pub radius: f32,
    pub material: Material,
}

impl Surface for Triangle {
    fn get_normal(&self, point: Point3d) -> Vec3d {
        todo!()
    }

    fn intersect(&self, ray: &Ray) -> Option<f32> {
        todo!()
    }
}
