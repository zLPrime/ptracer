use crate::primitives::Ray;
use crate::surface::Object;
use crate::vec3d::Point3d;
use crate::{Color, Material, MaterialKind};

use super::triangle::Triangle;
use super::Surface;

use core::f32;
use std::fs::read_to_string;
use std::mem::swap;

#[derive(Debug)]
struct BoundingBox {
    min: Point3d,
    max: Point3d,
}

pub struct Mesh {
    triangles: Vec<Triangle>,
    bounding_box: BoundingBox,
}

impl BoundingBox {
    pub fn new() -> BoundingBox {
        BoundingBox {
            min: Point3d {
                x: 0f32,
                y: 0f32,
                z: 0f32,
            },
            max: Point3d {
                x: 0f32,
                y: 0f32,
                z: 0f32,
            },
        }
    }
}

impl Mesh {
    pub fn import(path: &str) -> Mesh {
        let content = read_to_string(path).expect(format!("Could not read file {path}.").as_str());
        let mut vertices: Vec<Point3d> = Vec::new();
        let mut triangles: Vec<Triangle> = Vec::new();
        let mut bounding_box: BoundingBox = BoundingBox::new();
        for line in content.lines() {
            let mut split = line.split(' ');
            match split.next().unwrap() {
                "v" => {
                    let x = split.next().unwrap();
                    let y = split.next().unwrap();
                    let z = split.next().unwrap();
                    let vertex =
                        Point3d::new(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap());

                    // updating bounding box
                    if vertex.x < bounding_box.min.x {
                        bounding_box.min.x = vertex.x;
                    } else if vertex.x > bounding_box.max.x {
                        bounding_box.max.x = vertex.x;
                    }
                    if vertex.y < bounding_box.min.y {
                        bounding_box.min.y = vertex.y;
                    } else if vertex.y > bounding_box.max.y {
                        bounding_box.max.y = vertex.y;
                    }
                    if vertex.z < bounding_box.min.z {
                        bounding_box.min.z = vertex.z;
                    } else if vertex.z > bounding_box.max.z {
                        bounding_box.max.z = vertex.z;
                    }

                    vertices.push(vertex);
                    assert!(split.next() == None)
                }
                "f" => {
                    let a_i = parse_vertex_index(split.next().unwrap());
                    let b_i = parse_vertex_index(split.next().unwrap());
                    let c_i = parse_vertex_index(split.next().unwrap());
                    assert!(split.next() == None, "Only triagonal faces are supported!");
                    let a = vertices[a_i];
                    let b = vertices[b_i];
                    let c = vertices[c_i];
                    let material = Material {
                        material_kind: MaterialKind::Diffuse,
                        color: Color::new(0.8, 0.8, 0.8),
                    };
                    let triangle = Triangle::new(a, b, c, material);
                    triangles.push(triangle);
                }
                _ => {} // ignore other objects
            }
        }
        println!("{:?}", bounding_box);
        Mesh {
            triangles,
            bounding_box,
        }
    }
}

impl Object for Mesh {
    fn intersect(&self, ray: &Ray) -> (f32, Option<&Triangle>) {
        let mut closest_distance = f32::MAX;
        let mut closest_triangle = None;

        // check if ray hits bounding box
        if hits_boudning_box(ray, &self.bounding_box) {
            for triangle in &self.triangles {
                match triangle.intersect(ray) {
                    Some(distance) => {
                        if distance < closest_distance {
                            closest_distance = distance;
                            closest_triangle = Some(triangle);
                        }
                    }
                    None => continue,
                }
            }
            //println!("hit")
        } else {
            //println!("miss")
        }
        return (closest_distance, closest_triangle);
    }
}

fn parse_vertex_index(str: &str) -> usize {
    str.split('/').next().unwrap().parse::<usize>().unwrap() - 1
}

fn hits_boudning_box(ray: &Ray, bounding_box: &BoundingBox) -> bool {
    let mut min_t = f32::MIN;
    let mut max_t = f32::MAX;
    let res = intersect_one_dimention(
        &mut min_t,
        &mut max_t,
        ray.direction.x,
        bounding_box.min.x,
        bounding_box.max.x,
        ray.origin.x,
    ) && intersect_one_dimention(
        &mut min_t,
        &mut max_t,
        ray.direction.y,
        bounding_box.min.y,
        bounding_box.max.y,
        ray.origin.y,
    ) && intersect_one_dimention(
        &mut min_t,
        &mut max_t,
        ray.direction.z,
        bounding_box.min.z,
        bounding_box.max.z,
        ray.origin.z,
    ) && max_t > 0.;
    //if !res {panic!()}
    return res;
}

fn intersect_one_dimention(
    min_t: &mut f32,
    max_t: &mut f32,
    direction_i: f32,
    min_i: f32,
    max_i: f32,
    origin_i: f32,
) -> bool {
    if direction_i != 0. {
        let min_i_aligned = min_i - origin_i;
        let max_i_aligned = max_i - origin_i;
        let direction_i_inv = 1. / direction_i;
        let mut new_min_t = min_i_aligned * direction_i_inv;
        let mut new_max_t = max_i_aligned * direction_i_inv;
        if new_min_t > new_max_t {
            swap(&mut new_min_t, &mut new_max_t)
        };
        *min_t = f32::max(*min_t, new_min_t);
        *max_t = f32::min(*max_t, new_max_t);
        return max_t > min_t;
    } else {
        return origin_i <= max_i && origin_i >= min_i;
    }
}
