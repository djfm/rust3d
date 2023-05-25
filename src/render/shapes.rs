use crate::math::{Vec3, Mat3};

pub trait Shape {
    fn translate(&mut self, d_pos: &Vec3);
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction: direction.normalize() }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Diamond {
    pub center: Vec3,
    pub width: Vec3,
    pub height: Vec3,
}

#[derive(Debug)]
pub struct Intersection {
    pub point: Vec3,
    pub dist: f32,
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

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // println!("intersect {:?} with {:?}", ray, self);

        let mat = Mat3::from_cols(&self.width, &self.height, &ray.direction);
        let v = ray.origin - self.center + self.width / 2.0 + self.height / 2.0;

        // ray.origin - self.center = w * self.width + h * self.height - t * ray.direction

        if let Some(inv) = mat.invert() {
            let Vec3{x: w, y: h, z: neg_t} = inv * v;
            let t = -neg_t;

            if w >= -0.5 && w <= 0.5 && h >= 0.5 && h <= 0.5 && t >= 0.0 {
                Some(Intersection {
                    point: ray.origin + t * ray.direction,
                    dist: t,
                })
            } else {
                None
            }
        } else {
            None
        }
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
