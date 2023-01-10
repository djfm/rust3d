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

pub trait Shape {
    fn translate(&mut self, d_pos: &Vec3);
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
pub struct Diamond {
  pub center: Vec3,
  pub width: Vec3,
  pub height: Vec3,
}

impl Diamond {
  pub fn new(center: Vec3, width: Vec3, height: Vec3) -> Diamond {
    Diamond { center, width, height }
  }
}

impl Shape for Diamond {
  fn translate(&mut self, d_pos: &Vec3) {
    self.center = self.center + *d_pos;
  }
}

#[derive(Debug)]
pub struct Camera {
  pub position: Vec3,
  pub screen: Diamond,
}

impl Camera {
  pub fn new(position: Vec3, screen: Diamond) -> Camera {
    Camera { position, screen }
  }
}

pub struct Scene {
    pub camera: Camera,
    pub shapes: Vec<Box<dyn Shape>>,
}

impl Scene {
    pub fn new(camera: Camera) -> Scene {
        Scene {
            camera,
            shapes: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Shape>) -> &mut Self {
        self.shapes.push(object);
        self
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
