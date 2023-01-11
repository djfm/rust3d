use crate::math::Vec3;

pub trait Shape {
    fn translate(&mut self, d_pos: &Vec3);
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
