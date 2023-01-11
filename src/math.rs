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

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
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

impl ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vectors() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 2.0, 1.0);
        let actual = v1 + v2;
        let expected = Vec3::new(4.0, 4.0, 4.0);
        assert_eq!(actual, expected, "vector addition failed");
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
    fn test_scalar_mul_vector() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let actual = 3.0 * v;
        let expected = Vec3::new(3.0, 6.0, 9.0);
        assert_eq!(actual, expected, "scalar multiplication failed");
    }

    #[test]
    fn test_div_vector() {
        let v = Vec3::new(2.0, 4.0, 6.0);
        let actual = v / 2.0;
        let expected = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(actual, expected, "vector division failed");
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
}
