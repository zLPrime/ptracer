use std::ops;

pub struct Matrix3x3 {
    a11: f32, a12: f32, a13: f32,
    a21: f32, a22: f32, a23: f32,
    a31: f32, a32: f32, a33: f32,
}


pub struct Matrix3x1 {
    pub a31: f32,
    pub a11: f32,
    pub a21: f32,
}

impl Matrix3x3 {
    pub fn new(a11: f32, a12: f32, a13: f32,
        a21: f32, a22: f32, a23: f32,
        a31: f32, a32: f32, a33: f32) -> Matrix3x3 {
            Matrix3x3 {
                a11, a12, a13,
                a21, a22, a23,
                a31, a32, a33,
            }
        }
}

impl Matrix3x1 {
    pub fn new(
        a11: f32,
        a21: f32,
        a31: f32) -> Matrix3x1 {
            Matrix3x1 {
                a11,
                a21,
                a31,
            }
        }
}

impl ops::Mul<Matrix3x1> for Matrix3x3 {
    type Output = Matrix3x1;

    fn mul(self, rhs: Matrix3x1) -> Self::Output {
        let a11 = self.a11 * rhs.a11 + self.a12 * rhs.a21 + self.a13 * rhs.a31;
        let a21 = self.a21 * rhs.a11 + self.a22 * rhs.a21 + self.a23 * rhs.a31;
        let a31 = self.a31 * rhs.a11 + self.a32 * rhs.a21 + self.a33 * rhs.a31;

        Matrix3x1 {
            a11,
            a21,
            a31
        }
    }
}