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

fn compute(scene: &Scene, screen: &Display) -> Vec<Point> {
    let bottom_left = scene.camera.screen.center - scene.camera.screen.width / 2.0 - scene.camera.screen.height / 2.0;

    let mut points:Vec<Point> = Vec::new();

    for x in 0..screen.width {
        for y in 0..screen.height {
            let screen_pos = bottom_left + (x as f32 / screen.width as f32) * scene.camera.screen.width + (y as f32 / screen.height as f32) * scene.camera.screen.height;

            let ray = Ray::new(
                screen_pos,
                screen_pos - scene.camera.position,
            );

            for shape in &scene.shapes {
                if let Some(intersection) = shape.intersect(&ray) {
                    points.push(
                        Point::new(
                            x as i32,
                            (screen.height - y) as i32,
                            compute_color(&ray, &intersection)
                        )
                    );
                }
            }
        }
    }

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
