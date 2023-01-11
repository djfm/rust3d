mod render;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use render::{render, Display};
use render::shapes::{Scene, Vec3, Diamond, Camera, Shape};

pub fn main() {
    let width = 800;
    let height = 600;

    let ratio = height as f32 / width as f32;
    let screen_width = width as f32 / 10.0;
    let screen_height = screen_width * ratio;
    let focal = 20.0;

    let dz = Vec3::new(0.0, 0.0, 1.0);

    let origin = Vec3::new(0.0, 0.0, -100.0);
    let screen = Diamond::new(
        Vec3::new(-screen_width / 2.0, -screen_height / 2.0, origin.z - focal),
      Vec3::new(0.0, screen_width, 0.0),
      Vec3::new(0.0, 100.0, screen_height)
    );

    let mut rect = screen.clone();
    rect.translate(&(2.0 * focal * dz));
    let camera = Camera::new(origin, screen);
    println!("Camera: {:?}", camera);
    println!("Rect: {:?}", rect);

    let mut scene = Scene::new(camera);
    scene.add(Box::new(rect));

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", width, height)
    .position_centered()
    .build()
    .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    let mut display = Display::new(canvas, width, height);

    display.canvas.set_draw_color(Color::RGB(0, 255, 255));
    display.canvas.clear();
    display.canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // display.canvas.set_draw_color(Color::RGB(255, 255, 255));
        // display.canvas.draw_point(Point::new(100, 100)).unwrap();

        render(&mut scene, &mut display);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        display.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
