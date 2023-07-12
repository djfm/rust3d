use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use rust3d::render::{render, Display};
use rust3d::render::shapes::{Scene, Diamond, Camera, Sphere, Quad};
use rust3d::math::Vec3;

pub fn main() {
    let width = 800;
    let height = 600;

    let ratio = height as f32 / width as f32;
    let screen_width = width as f32 / 10.0;
    let screen_height = screen_width * ratio;
    let focal = 500.0;


    let screen_z = 0.0;
    let origin = Vec3::new(0.0, 0.0, screen_z - focal);
    let screen = Diamond::new(
        Vec3::new(origin.x, origin.y, screen_z),
        Vec3::new(screen_width, 0.0, 0.0),
        Vec3::new(0.0, screen_height, 0.0)
    );

    let camera = Camera::new(origin, screen);
    println!("Camera: {:?}", camera);

    let c = 10.0;
    let rect = Diamond::new(
        Vec3::new(-c, 0.0, 1.0),
        Vec3::new(c, 0.0, 0.0),
        Vec3::new(0.0, c, 0.0)
    );

    println!("Rect: {:?}", rect);

    let sphere = Sphere::new(Vec3::new(7.0, 15.0, 15.0), 12.0);

    let mut scene = Scene::new(camera);
    scene.add(Box::new(rect));
    scene.add(Box::new(sphere));

    let quad = Quad::iso(Vec3::new(30.0, -20.0, 80.0), 15.0);
    scene.add(Box::new(quad));

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", width, height)
    .position_centered()
    .build()
    .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    let mut display = Display::new(canvas, width, height);

    // display.canvas.set_draw_color(Color::RGB(0, 255, 255));
    // display.canvas.clear();
    // display.canvas.present();

    let mut frame_num = 0;
    let t_start = std::time::Instant::now();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        frame_num += 1;
        // display.canvas.set_draw_color(Color::RGB(255, 255, 255));
        // display.canvas.draw_point(Point::new(100, 100)).unwrap();

        render(&mut scene, &mut display);
        let t_elapsed = t_start.elapsed();
        let fps = frame_num as f64 / t_elapsed.as_secs_f64();

        println!("FPS: {}", fps);

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

        for shape in scene.shapes.iter_mut() {
            shape.rotate(0.0, 0.0, 0.01);
        }

        display.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
