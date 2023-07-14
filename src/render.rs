use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::video::Window;
use sdl2::render::Canvas;

pub mod objects;

use rayon::prelude::*;

use objects::{Scene, Ray};

use self::objects::Intersection;

impl objects::Color {
    pub fn as_sdl(colors: &Vec<objects::Color>) -> Color {
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;

        for color in colors {
            r += color.rgb.x;
            g += color.rgb.y;
            b += color.rgb.z;
        }

        let len = colors.len() as f32;

        Color::RGB((r * 255.0 / len) as u8, (g * 255.0 / len) as u8, (b * 255.0 / len) as u8)
    }
}

pub struct Display {
    pub canvas: Canvas<Window>,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct Point {
    point: SDLPoint,
    color: Color,
}

impl Point {
    pub fn new(x: i32, y: i32, color: Color) -> Point {
        Point {
            point: SDLPoint::new(x, y),
            color,
        }
    }
}

impl Display {
    pub fn new(canvas: Canvas<Window>, width: u32, height: u32) -> Display {
        Display { canvas, width, height }
    }
}

fn compute_color(ray: &Ray, intersection: &Intersection, scene: &Scene) -> Color {
    let a = ray.direction.dot(&intersection.normal);

    let c = (2.0 + a) / 2.0;
    let epsilon = 1.0;

    let mut colors: Vec<objects::Color> = scene.lights.iter().filter_map(|light| {
        let direction = light.origin() - intersection.point;

        let shadowed = scene.shapes.iter().any(|shape| {
            match shape.intersect(&Ray::new(intersection.point, direction)) {
                Some(intersection) => intersection.dist > epsilon,
                None => false,
            }
        });

        if shadowed {
            return None;
        }

        // light distance
        let d = direction.norm();

        Some(light.color.dim(d * d / 10000.0))
    }).collect();

    colors.push(objects::Color::new(c, c, c));

    return objects::Color::as_sdl(&colors);
}

fn slice(distance: u32, parts: u32) -> Vec<u32> {
    let length = (distance as f32 / parts as f32).floor() as u32;
    let delta = distance - length * parts;
    (0..parts).map(|i| length + if i < delta { 1 } else { 0 }).collect()
}

fn ranges(distance: u32, parts: u32) -> Vec<(u32, u32)> {
    let slices = slice(distance, parts);
    let mut ranges: Vec<(u32, u32)> = Vec::new();
    let mut start = 0;
    for slice in slices {
        ranges.push((start, start + slice));
        start += slice;
    }
    ranges
}

type Pair = (u32, u32);
type RectCoords = (Pair, Pair);
type Points1D = Vec<Point>;
type Points2D = Vec<Points1D>;
type Points3D = Vec<Points2D>;

fn compute(scene: &Scene, screen: &Display) -> Points3D {
    let bottom_left = scene.camera.screen.center - scene.camera.screen.width / 2.0 - scene.camera.screen.height / 2.0;

    let x_slices = ranges(screen.width, 4);
    let y_slices = ranges(screen.height, 4);

    let screen_width = screen.width as f32;
    let screen_height = screen.height as f32;
    let scene_width = scene.camera.screen.width;
    let scene_height = scene.camera.screen.height;
    let camera_pos = scene.camera.position;

    let t_start = std::time::Instant::now();
    let screen_parts: Vec<RectCoords> = x_slices.iter().flat_map(|x| {
        y_slices.iter().map(move |y| {
            (*x, *y)
        })
    }).collect();
    println!("Splitting screen: {:?}ms", t_start.elapsed().as_millis());

    let compute_x_y_pixel = |x: u32, y: u32| -> Option<Point> {
        let screen_pos = bottom_left + (x as f32 / screen_width) * scene_width + (y as f32 / screen_height as f32) * scene_height;

        let ray = Ray::new(
            screen_pos,
            screen_pos - camera_pos,
        );

        let mut intersections: Vec<Intersection> = Vec::new();

        for shape in &scene.shapes {
            let maybe_inter = shape.intersect(&ray);

            if let Some(intersection) = maybe_inter {
                intersections.push(
                    intersection
                );
            }
        }

        match Intersection::nearest(&mut intersections) {
            Some(intersection) => {
                Some(Point::new(
                    x as i32,
                    (screen_height - y as f32) as i32,
                    compute_color(&ray, &intersection, &scene)
                ))
            },
            None => None
        }
    };

    let screen_part_processor = |(bl, tr): &RectCoords| -> Points2D {
        (bl.0..bl.1).map(|x| {
            (tr.0..tr.1).map(|y| {
                compute_x_y_pixel(x, y)
            }).filter(|maybe_point| {
                maybe_point.is_some()
            }).map(|maybe_point| {
                maybe_point.unwrap()
            }).collect::<Points1D>()
        }).collect::<Points2D>()
    };

    let pixels: Points3D = screen_parts.par_iter().map(
        screen_part_processor
    ).collect();

    pixels
}

pub fn render(scene: &mut Scene, display: &mut Display) {
    let t_start = std::time::Instant::now();

    let points = compute(scene, display);

    let t_compute_ms = t_start.elapsed().as_millis();

    display.canvas.set_draw_color(Color::RGB(0, 0, 0));

    let t_display_ms = t_start.elapsed().as_millis() - t_compute_ms;

    display.canvas.clear();

    println!("Compute: {}ms, Display: {}ms", t_compute_ms, t_display_ms);

    for list_a in &points {
        for list_b in list_a {
            for point in list_b {
                display.canvas.set_draw_color(point.color);
                display.canvas.draw_point(point.point).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vec3;
    use crate::render::Intersection;

    #[test]
    fn test_intersections_order() {
        let a = Intersection {
            point: Vec3::new(0.0, 0.0, 0.0),
            dist: 1.0,
            normal: Vec3::new(0.0, 0.0, 0.0),
        };

        let b = Intersection {
            point: Vec3::new(0.0, 0.0, 0.0),
            dist: 2.0,
            normal: Vec3::new(0.0, 0.0, 0.0),
        };

        let mut inters = [a, b];

        let nearest = Intersection::nearest(&mut inters);
        assert_eq!(nearest.unwrap().dist, 1.0);
    }
}
