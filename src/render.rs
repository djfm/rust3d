use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::video::Window;
use sdl2::render::Canvas;

pub mod shapes;

use shapes::{Scene};

pub struct Display {
    pub canvas: Canvas<Window>,
    pub width: u32,
    pub height: u32,
}

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

fn compute(scene: &Scene, screen: &Display) -> Vec<Point> {
    vec![]
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
