use crate::surface::material;
use crate::vec3d::Point3d;
use crate::{Color, Material, MaterialKind};

use super::Surface;
use super::triangle::Triangle;

use std::fs::read_to_string;
use std::ops::Deref;

pub struct Mesh {
    triangles: Vec<Triangle>
}

impl Mesh {
    pub fn import(path: &str) -> Mesh {
        let content =
            read_to_string(path).expect(format!("Could not read file {path}.").as_str());
        let mut vertices:  Vec<Point3d> = Vec::new();
        let mut triangles: Vec<Triangle> = Vec::new();
        for line in content.lines() {
            let mut split = line.split(' ');
            match split.next().unwrap() {
                "v" => {
                    let x = split.next().unwrap();
                    let y = split.next().unwrap();
                    let z = split.next().unwrap();
                    let vertex =
                        Point3d::new(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap());
                    vertices.push(vertex);
                    assert!(split.next() == None)
                },
                "f" => {
                    let a_i = Mesh::parse_vertex_index(split.next().unwrap());
                    let b_i = Mesh::parse_vertex_index(split.next().unwrap());
                    let c_i = Mesh::parse_vertex_index(split.next().unwrap());
                    assert!(split.next() == None, "Only triagonal faces are supported!");
                    let a = vertices[a_i];
                    let b = vertices[b_i];
                    let c = vertices[c_i];
                    let material =
                        Material { material_kind: MaterialKind::Diffuse, color: Color::new(0.8, 0.8, 0.8)};
                    let triangle = Triangle::new(a, b, c, material);
                    triangles.push(triangle);
                },
                _ => {} // ignore other objects
            }
        }
        Mesh { triangles }
    }
    
    pub fn get_triangles(&self) -> Vec<Triangle> {
        self.triangles.to_vec()
    }
    
    fn parse_vertex_index(str: &str) -> usize {
        str.split('/').next().unwrap().parse::<usize>().unwrap() - 1
    }
}

impl Surface for Mesh {
    fn get_normal(&self, point: crate::vec3d::Point3d) -> crate::vec3d::Vec3d {
        todo!()
    }

    fn intersect(&self, ray: &crate::Ray) -> Option<f32> {
        todo!()
    }

    fn get_material(&self) -> crate::Material {
        todo!()
    }
}