use crate::primitives::Ray;
use crate::primitives::vec3d::{Vec3d, Point3d};
use crate::surface::triangle::Triangle;
use crate::Material;

pub mod material;
pub mod sphere;
pub mod triangle;
pub mod mesh;

pub trait Surface {
    fn get_normal(&self, point: Point3d) -> Vec3d;
    fn intersect(&self, ray: &Ray) -> Option<f32>;
    fn get_material(&self) -> Material;
}

pub trait Object {
    fn intersect(&self, ray: &Ray) -> (f32, Option<&Triangle>);
}