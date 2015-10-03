pub use std::f32::consts::PI;

#[derive(Debug, Copy, Clone)]
pub struct Matrix2d {
    pub data: [[f32; 3]; 3],
}

use super::vector::Vector2d;
impl Matrix2d {
    pub fn new() -> Matrix2d {
        Matrix2d::identity()
    }

    pub fn identity() -> Matrix2d {
        Matrix2d {
            data: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ]
        }
    }

    pub fn multiply_m(&self, other: &Matrix2d) -> Matrix2d {
        let mut result = Matrix2d {
            data: [[0.0; 3]; 3]
        };

        for row in 0..3 {
            for col in 0..3 {
                result.data[row][col] =  self.data[row][0]*other.data[0][col] + 
                                         self.data[row][1]*other.data[1][col] + 
                                         self.data[row][2]*other.data[2][col];
            }
        }

        result
    }

    pub fn multiply_v(&self, vector: &Vector2d) -> Vector2d {
        let (vx, vy) = vector.get_pos();

        let x = self.data[0][0]*vx + self.data[0][1]*vy + self.data[0][2];
        let y = self.data[1][0]*vx + self.data[1][1]*vy + self.data[1][2];
        let _w = self.data[2][0]*vx + self.data[2][1]*vy + self.data[2][2];
        
        Vector2d::new(x,y)
    }

    pub fn rotate(&self, theta: f32) -> Matrix2d {
        let mut result = Matrix2d::identity();
        
        result.data[0][0] = theta.cos();
        result.data[0][1] = -theta.sin();
        result.data[1][0] = theta.sin();
        result.data[1][1] = theta.cos();

        result
    }

    pub fn translate(&self, xtrans: f32, ytrans: f32) -> Matrix2d {
        let mut result = Matrix2d::identity();

        result.data[0][2] = xtrans;
        result.data[1][2] = ytrans;

        result
    }

    pub fn scale(&self, xscale: f32, yscale: f32) -> Matrix2d {
        let mut result = Matrix2d::identity();

        result.data[0][0] = xscale;
        result.data[1][1] = yscale;

        result
    }
}


use std::ops::Mul;
impl Mul<Matrix2d> for Matrix2d {
    type Output = Matrix2d;

    fn mul(self, other: Matrix2d) -> Matrix2d {
        self.multiply_m(&other)
    }

}

impl Mul<Vector2d> for Matrix2d {
    type Output = Vector2d;

    fn mul(self, other: Vector2d) -> Vector2d {
        self.multiply_v(&other)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix3d {
    pub data: [[f32; 4]; 4],
}

use super::vector::Vector3d;
impl Matrix3d {
    pub fn new() -> Matrix3d {
        Matrix3d::identity()
    }

    pub fn identity() -> Matrix3d {
        Matrix3d {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        }
    }

    pub fn multiply_m(&self, other: &Matrix3d) -> Matrix3d {
        let mut result = Matrix3d {
            data: [[0.0; 4]; 4]
        };

        for row in 0..4 {
            for col in 0..4 {
                result.data[row][col] =  self.data[row][0]*other.data[0][col] +
                                         self.data[row][1]*other.data[1][col] +
                                         self.data[row][2]*other.data[2][col] +
                                         self.data[row][3]*other.data[3][col];
            }
        }

        result
    }

    pub fn multiply_v(&self, other: &Vector3d) -> Vector3d {
        let (vx, vy, vz) = other.get_pos();

        let x = self.data[0][0]*vx + self.data[0][1]*vy + self.data[0][2]*vz + self.data[0][3];
        let y = self.data[1][0]*vx + self.data[1][1]*vy + self.data[1][2]*vz + self.data[1][3];
        let z = self.data[2][0]*vx + self.data[2][1]*vy + self.data[2][2]*vz + self.data[2][3];
        let _w = self.data[3][0]*vx + self.data[3][1]*vy + self.data[3][2]*vz + self.data[3][3];

        Vector3d::new(x, y, z)
    }

    pub fn rotate(&self, xtheta: f32, ytheta: f32, ztheta: f32) -> Matrix3d {
        let identity = Matrix3d::identity();
        let rotation = identity.rotate_x(xtheta).rotate_y(ytheta).rotate_z(ztheta);

        *self * rotation
    }

    pub fn rotate_x(&self, theta: f32) -> Matrix3d {
        let mut result = Matrix3d::identity();

        result.data[1][1] = theta.cos();
        result.data[1][2] = -theta.sin();
        result.data[2][1] = theta.sin();
        result.data[2][2] = theta.cos();

        *self * result
    }

    pub fn rotate_y(&self, theta: f32) -> Matrix3d {
        let mut result = Matrix3d::identity();

        result.data[0][0] = theta.cos();
        result.data[0][2] = theta.sin();
        result.data[2][0] = -theta.sin();
        result.data[2][2] = theta.cos();

        *self * result
    }

    pub fn rotate_z(&self, theta: f32) -> Matrix3d {
        let mut result = Matrix3d::identity();

        result.data[0][0] = theta.cos();
        result.data[0][1] = -theta.sin();
        result.data[1][0] = theta.sin();
        result.data[1][1] = theta.cos();

        *self * result
    }
}

impl Mul<Matrix3d> for Matrix3d {
    type Output = Matrix3d;

    fn mul(self, other: Matrix3d) -> Matrix3d {
        self.multiply_m(&other)
    }
}

impl Mul<Vector3d> for Matrix3d {
    type Output = Vector3d;

    fn mul(self, other: Vector3d) -> Vector3d {
        self.multiply_v(&other)
    }
}
