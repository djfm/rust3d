use rust3d::render::objects::Diamond;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use rust3d::render::{objects, objects::Camera};
use rust3d::render::{render, Display};
use rust3d::math::Vec3;

fn get_bad_first_scene(width: u32, height: u32) -> objects::Scene {
    let ratio = height as f32 / width as f32;
    let screen_width = width as f32 / 10.0;
    let screen_height = screen_width * ratio;
    let focal = 500.0;

    let screen_z = 0.0;
    let origin = Vec3::new(0.0, 0.0, screen_z - focal);
    let screen = objects::Diamond::new(
        Vec3::new(origin.x, origin.y, screen_z),
        Vec3::new(screen_width, 0.0, 0.0),
        Vec3::new(0.0, screen_height, 0.0)
    );

    let camera = Camera::new(origin, screen);
    println!("Camera: {:?}", camera);

    let c = 10.0;
    let rect = objects::Diamond::new(
        Vec3::new(-c, 0.0, 1.0),
        Vec3::new(c, 0.0, 0.0),
        Vec3::new(0.0, c, 0.0)
    );

    println!("Rect: {:?}", rect);

    let sphere = objects::Sphere::new(Vec3::new(7.0, 15.0, 15.0), 12.0);

    let mut scene = objects::Scene::new(camera);
    scene.add_object(Box::new(rect));
    scene.add_object(Box::new(sphere));

    let quad = objects::Quad::iso(Vec3::new(30.0, -20.0, 80.0), 15.0);
    scene.add_object(Box::new(quad));

    let light_a = objects::Light::new(
        Vec3::new(40.0, 40.0, -50.0),
        5.0,
        objects::Color::new(1.0, 0.0, 0.0)
    );

    scene.add_light(light_a);

    scene
}

fn get_test_scene(screen_width: u32, screen_height: u32) -> objects::Scene {
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let pov = origin - 10.0 * Vec3::new(0.0, 0.0, 1.0);

    let camera_screen = Diamond::new(
        origin,
        Vec3::new(screen_width as f32 / 10.0, 0.0, 0.0),
        Vec3::new(0.0, screen_height as f32 / 10.0, 0.0),
    );

    let mut scene = objects::Scene::new(Camera::new(pov, camera_screen));

    let quad = objects::Quad::iso(Vec3::new(0.0, 0.0, 10.0), 10.0);

    scene.add_object(Box::new(quad));

    scene
}

enum Scene {
    Bad,
    Test
}

fn get_scene(screen_width: u32, screen_height: u32, which: Scene) -> objects::Scene {
    match which {
        Scene::Bad => get_bad_first_scene(screen_width, screen_height),
        Scene::Test => get_test_scene(screen_width, screen_height),
    }
}

pub fn main() {
    let width = 800;
    let height = 600;

    let mut scene = get_scene(width, height, Scene::Test);

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

        println!("\nFPS: {}\n", fps);

        let a = std::f32::consts::PI / 180.0;
        let d = 10.0;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    scene.camera.translate(0.0, 0.0, d);
                    println!("Camera position: {:?}", scene.camera.position);
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    scene.camera.translate(0.0, 0.0, -d);
                    println!("Camera position: {:?}", scene.camera.position);
                },
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    scene.camera.translate(-d, 0.0, 0.0);
                    println!("Camera position: {:?}", scene.camera.position);
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    scene.camera.translate(d, 0.0, 0.0);
                    println!("Camera position: {:?}", scene.camera.position);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    scene.camera.rotate(a, 0.0, 0.0);
                    println!("Camera position: {:?}", scene.camera.position);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    scene.camera.rotate(-a, 0.0, 0.0);
                    println!("Camera position: {:?}", scene.camera.position);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    scene.camera.rotate(0.0, a, 0.0);
                    println!("Camera position: {:?}", scene.camera.position);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    scene.camera.rotate(0.0, -a, 0.0);
                    println!("Camera position: {:?}", scene.camera.position);
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        /*
            for shape in scene.shapes.iter_mut() {
                shape.rotate(0.0, 0.0, 0.01);
            }
        */

        display.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
