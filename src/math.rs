use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn norm2(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(&self) -> f32 {
        self.norm2().sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let norm = self.norm();
        Vec3 {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
        }
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> [Vec3; 2] {
        [
            Vec3 {
                x: self.y * other.z - self.z * other.y,
                y: self.z * other.x - self.x * other.z,
                z: self.x * other.y - self.y * other.x,
            }.normalize(),
            Vec3 {
                x: self.z * other.y - self.y * other.z,
                y: self.x * other.z - self.z * other.x,
                z: self.y * other.x - self.x * other.y,
            }.normalize(),
        ]
    }

    pub fn angle(&self, other: &Vec3) -> f32 {
        self.dot(other).acos()
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat3 {
    pub coords: [[f32; 3]; 3],
}

impl Mat3 {
    pub fn new(coords: [[f32; 3]; 3]) -> Mat3 {
        Mat3 { coords }
    }

    pub fn from_cols(a: &Vec3, b: &Vec3, c: &Vec3) -> Mat3 {
        Mat3 {
            coords: [
                [a.x, b.x, c.x],
                [a.y, b.y, c.y],
                [a.z, b.z, c.z],
            ],
        }
    }
}

impl ops::Add<Mat3> for Mat3 {
    type Output = Mat3;

    fn add(self, other: Mat3) -> Mat3 {
        Mat3 {
            coords: [
                [
                    self.coords[0][0] + other.coords[0][0],
                    self.coords[0][1] + other.coords[0][1],
                    self.coords[0][2] + other.coords[0][2],
                ],
                [
                    self.coords[1][0] + other.coords[1][0],
                    self.coords[1][1] + other.coords[1][1],
                    self.coords[1][2] + other.coords[1][2],
                ],
                [
                    self.coords[2][0] + other.coords[2][0],
                    self.coords[2][1] + other.coords[2][1],
                    self.coords[2][2] + other.coords[2][2],
                ],
            ],
        }
    }
}

impl ops::AddAssign<Mat3> for Mat3 {
    fn add_assign(&mut self, other: Mat3) {
        self.coords[0][0] += other.coords[0][0];
        self.coords[0][1] += other.coords[0][1];
        self.coords[0][2] += other.coords[0][2];
        self.coords[1][0] += other.coords[1][0];
        self.coords[1][1] += other.coords[1][1];
        self.coords[1][2] += other.coords[1][2];
        self.coords[2][0] += other.coords[2][0];
        self.coords[2][1] += other.coords[2][1];
        self.coords[2][2] += other.coords[2][2];
    }
}

impl ops::Sub<Mat3> for Mat3 {
    type Output = Mat3;

    fn sub(self, other: Mat3) -> Mat3 {
        Mat3 {
            coords: [
                [
                    self.coords[0][0] - other.coords[0][0],
                    self.coords[0][1] - other.coords[0][1],
                    self.coords[0][2] - other.coords[0][2],
                ],
                [
                    self.coords[1][0] - other.coords[1][0],
                    self.coords[1][1] - other.coords[1][1],
                    self.coords[1][2] - other.coords[1][2],
                ],
                [
                    self.coords[2][0] - other.coords[2][0],
                    self.coords[2][1] - other.coords[2][1],
                    self.coords[2][2] - other.coords[2][2],
                ],
            ],
        }
    }
}

impl ops::SubAssign<Mat3> for Mat3 {
    fn sub_assign(&mut self, other: Mat3) {
        self.coords[0][0] -= other.coords[0][0];
        self.coords[0][1] -= other.coords[0][1];
        self.coords[0][2] -= other.coords[0][2];
        self.coords[1][0] -= other.coords[1][0];
        self.coords[1][1] -= other.coords[1][1];
        self.coords[1][2] -= other.coords[1][2];
        self.coords[2][0] -= other.coords[2][0];
        self.coords[2][1] -= other.coords[2][1];
        self.coords[2][2] -= other.coords[2][2];
    }
}

impl ops::Mul<Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: Mat3) -> Self::Output {
        let mut result = Mat3::new([[0.0; 3]; 3]);
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    result.coords[i][j] += self.coords[i][k] * rhs.coords[k][j];
                }
            }
        }
        result
    }
}

impl ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.coords[0][0] * rhs.x + self.coords[0][1] * rhs.y + self.coords[0][2] * rhs.z,
            y: self.coords[1][0] * rhs.x + self.coords[1][1] * rhs.y + self.coords[1][2] * rhs.z,
            z: self.coords[2][0] * rhs.x + self.coords[2][1] * rhs.y + self.coords[2][2] * rhs.z,
        }
    }
}

impl ops::Mul<Mat3> for f32 {
    type Output = Mat3;

    fn mul(self, rhs: Mat3) -> Self::Output {
        Mat3 {
            coords: [
                [
                    self * rhs.coords[0][0],
                    self * rhs.coords[0][1],
                    self * rhs.coords[0][2],
                ],
                [
                    self * rhs.coords[1][0],
                    self * rhs.coords[1][1],
                    self * rhs.coords[1][2],
                ],
                [
                    self * rhs.coords[2][0],
                    self * rhs.coords[2][1],
                    self * rhs.coords[2][2],
                ],
            ],
        }
    }
}

impl ops::Div<f32> for Mat3 {
    type Output = Mat3;

    fn div(self, rhs: f32) -> Self::Output {
        Mat3 {
            coords: [
                [
                    self.coords[0][0] / rhs,
                    self.coords[0][1] / rhs,
                    self.coords[0][2] / rhs,
                ],
                [
                    self.coords[1][0] / rhs,
                    self.coords[1][1] / rhs,
                    self.coords[1][2] / rhs,
                ],
                [
                    self.coords[2][0] / rhs,
                    self.coords[2][1] / rhs,
                    self.coords[2][2] / rhs,
                ],
            ],
        }
    }
}

pub static ID_MAT3: Mat3 = Mat3 {
    coords: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
};

pub static ZERO_MAT3: Mat3 = Mat3 {
    coords: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
};

impl Mat3 {
    pub fn equals(&self, other: &Mat3, epsilon: f32) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if (self.coords[i][j] - other.coords[i][j]).abs() > epsilon {
                    return false;
                }
            }
        }
        true
    }

    pub fn transpose(&self) -> Mat3 {
        Mat3 {
            coords: [
                [self.coords[0][0], self.coords[1][0], self.coords[2][0]],
                [self.coords[0][1], self.coords[1][1], self.coords[2][1]],
                [self.coords[0][2], self.coords[1][2], self.coords[2][2]],
            ],
        }
    }

    pub fn sub_factor(&self, row: usize, col: usize) -> f32 {
        let row_indices = if row == 0 {
            [1, 2]
        } else if row == 1 {
            [0, 2]
        } else {
            [0, 1]
        };

        let col_indices = if col == 0 {
            [1, 2]
        } else if col == 1 {
            [0, 2]
        } else {
            [0, 1]
        };

        let sign = if (row + col) % 2 == 0 { 1.0 } else { -1.0 };

        sign * (
            self.coords[row_indices[0]][col_indices[0]] * self.coords[row_indices[1]][col_indices[1]]
            -
            self.coords[row_indices[0]][col_indices[1]] * self.coords[row_indices[1]][col_indices[0]]
        )
    }

    pub fn invert(&self) -> Option<Mat3> {

        let co_matrix = Mat3 {
            coords: [
                [self.sub_factor(0, 0), self.sub_factor(0, 1), self.sub_factor(0, 2)],
                [self.sub_factor(1, 0), self.sub_factor(1, 1), self.sub_factor(1, 2)],
                [self.sub_factor(2, 0), self.sub_factor(2, 1), self.sub_factor(2, 2)],
            ],
        };

        let co_matrix_t = co_matrix.transpose();

        let determinant =
              self.coords[0][0] * co_matrix_t.coords[0][0]
            + self.coords[0][1] * co_matrix_t.coords[1][0]
            + self.coords[0][2] * co_matrix_t.coords[2][0];

        if determinant == 0.0 {
            None
        } else {
            Some(co_matrix_t / determinant)
        }
    }
}

pub struct Line {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Line {
    pub fn new(origin: Vec3, direction: Vec3) -> Line {
        Line {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_add_vectors() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 2.0, 1.0);
        let actual = v1 + v2;
        let expected = Vec3::new(4.0, 4.0, 4.0);
        assert_eq!(actual, expected, "vector addition failed");
    }

    #[test]
    fn test_vector_add_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 2.0, 1.0);
        v1 += v2;
        let expected = Vec3::new(4.0, 4.0, 4.0);
        assert_eq!(v1, expected, "vector addition failed");
    }

    #[test]
    fn test_sub_vectors() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 2.0, 1.0);
        let actual = v1 - v2;
        let expected = Vec3::new(-2.0, 0.0, 2.0);
        assert_eq!(actual, expected, "vector subtraction failed");
    }

    #[test]
    fn test_vector_sub_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 2.0, 1.0);
        v1 -= v2;
        let expected = Vec3::new(-2.0, 0.0, 2.0);
        assert_eq!(v1, expected, "vector subtraction failed");
    }

    #[test]
    fn test_scalar_mul_vector() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let actual = 3.0 * v;
        let expected = Vec3::new(3.0, 6.0, 9.0);
        assert_eq!(actual, expected, "scalar multiplication failed");
    }

    #[test]
    fn test_vector_mul_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v *= 3.0;
        let expected = Vec3::new(3.0, 6.0, 9.0);
        assert_eq!(v, expected, "scalar multiplication failed");
    }

    #[test]
    fn test_div_vector() {
        let v = Vec3::new(2.0, 4.0, 6.0);
        let actual = v / 2.0;
        let expected = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(actual, expected, "vector division failed");
    }

    #[test]
    fn test_vector_div_assign() {
        let mut v = Vec3::new(2.0, 4.0, 6.0);
        v /= 2.0;
        let expected = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v, expected, "vector division failed");
    }

    #[test]
    fn test_add_matrices() {
        let m = Mat3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let n = Mat3::new([[3.0, 2.0, 1.0], [6.0, 5.0, 4.0], [9.0, 8.0, 7.0]]);
        let actual = m + n;
        let expected = Mat3::new([[4.0, 4.0, 4.0], [10.0, 10.0, 10.0], [16.0, 16.0, 16.0]]);
        assert_eq!(actual, expected, "matrix addition failed");
    }

    #[test]
    fn test_sub_matrices() {
        let m = Mat3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let n = Mat3::new([[3.0, 2.0, 1.0], [6.0, 5.0, 4.0], [9.0, 8.0, 7.0]]);
        let actual = m - n;
        let expected = Mat3::new([[-2.0, 0.0, 2.0], [-2.0, 0.0, 2.0], [-2.0, 0.0, 2.0]]);
        assert_eq!(actual, expected, "matrix subtraction failed");
    }

    #[test]
    fn test_scalar_matrix_mul() {
        let m = Mat3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let actual = 2.0 * m;
        let expected = Mat3::new([[2.0, 4.0, 6.0], [8.0, 10.0, 12.0], [14.0, 16.0, 18.0]]);
        assert_eq!(actual, expected, "scalar multiplication failed");
    }

    #[test]
    fn test_matrix_div() {
        let m = Mat3::new([[2.0, 4.0, 6.0], [12.0, 10.0, 8.0], [14.0, 16.0, 18.0]]);
        let actual = m / 2.0;
        let expected = Mat3::new([[1.0, 2.0, 3.0], [6.0, 5.0, 4.0], [7.0, 8.0, 9.0]]);
        assert_eq!(actual, expected, "matrix division failed");
    }

    #[test]
    fn test_matrix_vector_mul() {
        let m = Mat3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let v = Vec3::new(1.0, 2.0, 3.0);
        let actual = m * v;
        let expected = Vec3::new(14.0, 32.0, 50.0);
        assert_eq!(actual, expected, "matrix-vector multiplication failed");
    }

    #[test]
    fn test_matrix_inverse_none() {
        let m = Mat3::new([[1.0, 2.0, 3.0], [2.0, 4.0, 6.0], [7.0, 8.0, 9.0]]);
        let actual = m.invert();
        assert!(actual.is_none(), "matrix inverse failed");
    }

    #[test]
    fn test_matrix_transpose() {
        let m = Mat3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let actual = m.transpose();
        let expected = Mat3::new([[1.0, 4.0, 7.0], [2.0, 5.0, 8.0], [3.0, 6.0, 9.0]]);
        assert_eq!(actual, expected, "matrix transpose failed");
    }

    #[test]
    fn test_matrix_inverse() {
        let m = Mat3::new([[1.0, 3.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let inv = m.invert().unwrap();
        let result = m * inv;
        assert_eq!(result, ID_MAT3, "matrix inverse failed");
    }

    #[test]
    fn test_many_matrix_inverses() {
        let mut rng = rand::thread_rng();
        let tests_count = 100;
        let mut inverted_count = 0;

        for _ in 0..tests_count {
            let m = Mat3::new([
                [rng.gen(), rng.gen(), rng.gen()],
                [rng.gen(), rng.gen(), rng.gen()],
                [rng.gen(), rng.gen(), rng.gen()]
            ]);

            let m_inv = m.invert();

            match m_inv {
                Some(inv) => {
                    let inv_m = inv * m;
                    let m_inv = m * inv;
                    assert!(inv_m.equals(&ID_MAT3, 0.0001), "matrix inverse failed");
                    assert!(m_inv.equals(&ID_MAT3, 0.0001), "matrix inverse failed");
                    inverted_count += 1;
                },
                None => {}
            }

            assert!(inverted_count > 0, "matrix inverse failed");
        }
    }

    #[test]
    fn test_vector_dot_product() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let actual = v1.dot(&v2);
        let expected = 32.0;
        assert_eq!(actual, expected, "vector dot product failed");
    }

}
