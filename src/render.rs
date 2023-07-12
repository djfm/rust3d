use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::video::Window;
use sdl2::render::Canvas;

pub mod shapes;

use shapes::{Scene, Ray};

use self::shapes::Intersection;

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

fn compute_color(ray: &Ray, intersection: &Intersection) -> Color {
    let a = ray.direction.dot(&intersection.normal);

    let c = (255.0 - a.abs() * 127.0) as u8;

    return Color::RGB(c, c, c);
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

type ScreenRect = ((u32, u32), (u32, u32));

fn compute(scene: &Scene, screen: &Display) -> Vec<Point> {
    let bottom_left = scene.camera.screen.center - scene.camera.screen.width / 2.0 - scene.camera.screen.height / 2.0;

    let mut points:Vec<Point> = Vec::new();

    let x_slices = ranges(screen.width, 8);
    let y_slices = ranges(screen.height, 8);

    let rects: Vec<ScreenRect> = x_slices.iter().flat_map(|x| {
        y_slices.iter().map(move |y| {
            (*x, *y)
        })
    }).collect();

    rects.iter().for_each(|(bl, tr)| {
        for x in bl.0..bl.1 {
            for y in tr.0..tr.1 {
                let screen_pos = bottom_left + (x as f32 / screen.width as f32) * scene.camera.screen.width + (y as f32 / screen.height as f32) * scene.camera.screen.height;

                let ray = Ray::new(
                    screen_pos,
                    screen_pos - scene.camera.position,
                );

                let mut intersections: Vec<Intersection> = Vec::new();

                for shape in &scene.shapes {
                    if let Some(intersection) = shape.intersect(&ray) {
                        intersections.push(
                            intersection
                        );
                    }
                }

                match Intersection::nearest(&mut intersections) {
                    Some(intersection) => {
                        points.push(Point::new(
                            x as i32,
                            (screen.height - y) as i32,
                            compute_color(&ray, &intersection)
                        ))
                    },
                    None => {}
                }
            }
        }
    });

    points
}

pub fn render(scene: &mut Scene, display: &mut Display) {
    let points = compute(scene, display);

    display.canvas.set_draw_color(Color::RGB(0, 0, 0));
    display.canvas.clear();

    for point in points {
        display.canvas.set_draw_color(point.color);
        display.canvas.draw_point(point.point).unwrap();
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
