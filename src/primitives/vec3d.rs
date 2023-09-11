use std::ops;

use rand::{thread_rng, Rng};

use super::matrix::{Matrix3x3, Matrix3x1};

#[derive(Debug,Copy,Clone)]
pub struct Point3d {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

pub type Vec3d = Point3d;

impl Vec3d {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3d {
        Vec3d { x, y, z }
    }

    pub fn len_squared(&self) -> f32
    {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn normalize(&self) -> Vec3d {
        *self * (1. / self.len())
    }

    pub fn cross(&self, other: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn rotate_x(&self, theta: f32) -> Vec3d {
        let rotate_matrix = Matrix3x3::new(
            theta.cos(), -(theta.sin()), 0.,
            theta.sin(), theta.cos(), 0.,
            0., 0., 1.
        );

        let vec_matrix = Matrix3x1::new(
            self.x,
            self.y,
            self.z
        );

        let result_matrix = rotate_matrix * vec_matrix;

        Vec3d::new(result_matrix.a11, result_matrix.a21, result_matrix.a31)
    }

    pub fn random() -> Vec3d {
        let mut rng = thread_rng();
        Vec3d::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
    }
}

impl ops::Mul<f32> for Vec3d {
    type Output = Vec3d;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3d {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Div<f32> for Vec3d {
    type Output = Vec3d;
    fn div(self, rhs: f32) -> Self::Output {
        let div = 1. / rhs;
        self * div
    }
}

impl ops::Add<Vec3d> for Vec3d {
    type Output = Vec3d;
    fn add(self, rhs: Vec3d) -> Self::Output {
        Vec3d { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl ops::Sub<Vec3d> for Vec3d {
    type Output = Vec3d;
    fn sub(self, rhs: Vec3d) -> Self::Output {
        Vec3d { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl ops::Mul<Vec3d> for Vec3d {
    type Output = f32;
    fn mul(self, rhs: Vec3d) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}


#[cfg(test)]
mod tests {
    use crate::Vec3d;

    #[test]
    fn len_squared() {
        let vec3d = Vec3d {
            x: 1.,
            y: 1.,
            z: 1.,
        };
        let len_squared = vec3d.len_squared();

        assert!(len_squared == 3.);
    }

    #[test]
    fn len() {
        let vec3d = Vec3d {
            x: 1.,
            y: 1.,
            z: 1.,
        };
        let len_squared = vec3d.len();

        assert!(len_squared == (3. as f32).sqrt());
    }

    #[test]
    fn normalize() {
        let vec3d = Vec3d {
            x: 1.,
            y: 1.,
            z: 1.,
        };
        let normalized = vec3d.normalize();

        assert!(normalized.x == (3. as f32).sqrt()/3.);
        assert!(normalized.y == (3. as f32).sqrt()/3.);
        assert!(normalized.z == (3. as f32).sqrt()/3.);
    }

    #[test]
    fn cross1() {
        let vec1 = Vec3d {
            x: 0.,
            y: 1.,
            z: 0.,
        };

        let vec2 = Vec3d {
            x: 1.,
            y: 0.,
            z: 0.,
        };

        let cross = vec1.cross(&vec2);

        assert!(cross.x == 0.);
        assert!(cross.y == 0.);
        assert!(cross.z == -1.);
    }

    #[test]
    fn cross2() {
        let vec1 = Vec3d {
            x: 1.,
            y: 0.,
            z: 0.,
        };

        let vec2 = Vec3d {
            x: 0.,
            y: 1.,
            z: 0.,
        };

        let cross = vec1.cross(&vec2);

        assert!(cross.x == 0.);
        assert!(cross.y == 0.);
        assert!(cross.z == 1.);
    }
}
