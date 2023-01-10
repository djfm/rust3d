#[derive(Debug)]
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

#[derive(Debug)]
pub struct Rectangle {
  pub center: Vec3,
  pub width: Vec3,
  pub height: Vec3,
}

impl Rectangle {
  pub fn new(center: Vec3, width: Vec3, height: Vec3) -> Rectangle {
    Rectangle { center, width, height }
  }
}

#[derive(Debug)]
pub struct Camera {
  pub position: Vec3,
  pub screen: Rectangle,
}

impl Camera {
  pub fn new(position: Vec3, screen: Rectangle) -> Camera {
    Camera { position, screen }
  }
}
