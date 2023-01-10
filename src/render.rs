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

pub fn render(scene: &Scene, screen: &Display) {
    let points = compute(scene, screen);
}
