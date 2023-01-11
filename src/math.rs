use std::ops;

#[derive(Debug, Copy, Clone)]
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

  impl ops::Div<Vec3> for f32 {
      type Output = Vec3;

      fn div(self, other: Vec3) -> Vec3 {
          Vec3 {
          x: self / other.x,
          y: self / other.y,
          z: self / other.z,
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

#[derive(Debug, Clone, Copy)]
pub struct Mat3 {
    pub coords: [[f32; 3]; 3],
}

impl Mat3 {
    pub fn new(coords: [[f32; 3]; 3]) -> Mat3 {
        Mat3 { coords }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        let m = Mat3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        println!("{:?}", m);
    }
}
