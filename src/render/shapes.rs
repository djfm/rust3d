use crate::math::{Vec3, Mat3};

pub trait Shape {
    fn translate(&mut self, d_pos: &Vec3);
    fn rotate(&mut self, theta_x: f32, theta_y: f32, theta_z: f32);
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

impl Default for Diamond {
    fn default() -> Self {
        Diamond {
            center: Vec3::new(0.0, 0.0, 0.0),
            width: Vec3::new(1.0, 0.0, 0.0),
            height: Vec3::new(0.0, 1.0, 0.0),
        }
    }
}

pub struct Quad {
    pub center: Vec3,
    pub width: Vec3,
    pub height: Vec3,
    pub depth: Vec3,
}

impl Quad {
    pub fn new(center: Vec3, width: Vec3, height: Vec3, depth: Vec3) -> Quad {
        Quad { center, width, height, depth }
    }

    pub fn iso(center: Vec3, side: f32) -> Quad {
        let width = Vec3::new(side, 0.0, 0.0);
        let height = Vec3::new(0.0, side, 0.0);
        let depth = Vec3::new(0.0, 0.0, side);

        Quad {
            center: center - width / 2.0 - height / 2.0 - depth / 2.0,
            width,
            height,
            depth,
        }
    }
}

impl Shape for Quad {
    fn translate(&mut self, d_pos: &Vec3) {
        self.center = self.center + *d_pos;
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut intersections = [
            Diamond::new(self.center - self.width / 2.0, self.height, self.depth),
            Diamond::new(self.center + self.width / 2.0, self.height, self.depth),
            Diamond::new(self.center - self.height / 2.0, self.width, self.depth),
            Diamond::new(self.center + self.height / 2.0, self.width, self.depth),
            Diamond::new(self.center - self.depth / 2.0, self.width, self.height),
            Diamond::new(self.center + self.depth / 2.0, self.width, self.height),
        ]
            .map(|diamond| diamond.intersect(ray))
            .iter()
            .filter(|i| i.is_some())
            .map(|i| i.unwrap())
            .collect::<Vec<_>>();

        Intersection::nearest(&mut intersections)
    }

    fn rotate(&mut self, theta_x: f32, theta_y: f32, theta_z: f32) {
        let mat = Mat3::rot_x_y_z(theta_x, theta_y, theta_z);
        self.width = mat * (self.width - self.center) + self.center;
        self.height = mat * (self.height - self.center) + self.center;
        self.depth = mat * (self.depth - self.center) + self.center;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Shape for Sphere {
    fn translate(&mut self, d_pos: &Vec3) {
        self.center = self.center + *d_pos;
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // ||ray.origin + t * ray.direction - self.center||^2 = self.radius^2
        let delta_o = ray.origin - self.center;
        let v = ray.direction;

        let a = v.norm2();
        let b = 2.0 * delta_o.dot(&v);
        let c = delta_o.norm2() - self.radius * self.radius;

        let delta = b * b - 4.0 * a * c;

        if delta > 0.0 {
            let candidates = [
                (-b - delta.sqrt()) / (2.0 * a),
                (-b + delta.sqrt()) / (2.0 * a),
            ];

            let min = if candidates[0] < candidates[1] { candidates[0] } else { candidates[1] };

            let point = ray.origin + min * ray.direction;

            Some(Intersection {
                point,
                dist: min,
                normal: (point - self.center).normalize(),
            })
        } else if delta == 0.0 {
            let t = -b / (2.0 * a);
            let point = ray.origin + t * ray.direction;

            Some(Intersection {
                point,
                dist: t,
                normal: (point - self.center).normalize(),
            })
        } else {
            None
        }
    }

    fn rotate(&mut self, _theta_x: f32, _theta_y: f32, _theta_z: f32) {
        // nothing to do fow now as spheres are homogeneous
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub point: Vec3,
    pub dist: f32,
    pub normal: Vec3,
}

impl Intersection {
    pub fn nearest(intersections: &mut[Intersection]) -> Option<Intersection> {
        if intersections.is_empty() {
            None
        } else {
            intersections.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());
            Some(intersections[0])
        }
    }
}

impl Diamond {
    pub fn new(center: Vec3, width: Vec3, height: Vec3) -> Diamond {
        Diamond { center, width, height, ..Default::default() }
    }
}

impl Shape for Diamond {
    fn translate(&mut self, d_pos: &Vec3) {
        self.center = self.center + *d_pos;
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // ray.origin + t * ray.direction = self.center + w * self.width + h * self.height
        // ray.origin - self.center = w * self.width + h * self.height - t * ray.direction
        let delta_o = ray.origin - self.center;
        let mat = Mat3::from_cols(&self.width, &self.height, &ray.direction);
        // delta_o =  mat * Vec3::new(w, h, -t)
        // if invertible: mat^(-1) * delta_o = Vec3::new(w, h, -t)

        if let Some(inv) = mat.invert() {
            let Vec3{x: w, y: h, z: neg_t} = inv * delta_o;
            let t = -neg_t;

            if w >= -0.5 && w <= 0.5 && h >= -0.5 && h <= 0.5 && t >= 0.0  {
                let cross_products = self.width.cross(&self.height);

                let normal = if cross_products[0].dot(&ray.direction) > 0.0 {
                    cross_products[0]
                } else {
                    cross_products[1]
                };

                Some(Intersection {
                    point: ray.origin + t * ray.direction,
                    dist: t,
                    normal,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn rotate(&mut self, theta_x: f32, theta_y: f32, theta_z: f32) {
        let mat = Mat3::rot_x_y_z(theta_x, theta_y, theta_z);
        self.width = mat * (self.width - self.center) + self.center;
        self.height = mat * (self.height - self.center) + self.center;
    }
}

pub enum BasicShape {
    Sphere(Sphere),
    Diamond(Diamond),
    Quad(Quad),
}

impl BasicShape {
    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            BasicShape::Sphere(ref sphere) => sphere.intersect(ray),
            BasicShape::Diamond(ref diamond) => diamond.intersect(ray),
            BasicShape::Quad(ref quad) => quad.intersect(ray),
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
    pub shapes: Vec<BasicShape>,
}

impl Scene {
    pub fn new(camera: Camera) -> Scene {
        Scene {
            camera,
            shapes: Vec::new(),
        }
    }

    pub fn add(&mut self, object: BasicShape) -> &mut Self {
        self.shapes.push(object);
        self
    }
}
