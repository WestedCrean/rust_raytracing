mod camera;
mod intersections;
mod lights;
mod ray;
mod scene;
mod shapes;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::render::Canvas;
use sdl2::video::Window;

//use glam::Mat4;

use crate::intersections::nearest_intersected_object;
use rand::Rng;
// use crate::lights::PositionalLight;
use crate::camera::Camera;
use crate::scene::Scene;
use crate::shapes::Sphere;
use nalgebra::Vector3;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

const BACKGROUND_COLOR: pixels::Color = pixels::Color::RGB(243, 183, 127);

#[derive(Debug, Clone)]
struct DrawSceneError;

fn initialize_scene() -> Scene {
    let mut scene = Scene::default();
    scene.push(Sphere::new(
        Vector3::new(-0.2, 0.0, -1.0),
        0.7,
        Vector3::new(218.0, 255.0, 63.0),
    ));
    scene.push(Sphere::new(
        Vector3::new(400.0, 400.0, 51.0),
        5.0,
        Vector3::new(218.0, 255.0, 63.0),
    ));
    scene.push(Sphere::new(
        Vector3::new(0.1, -0.3, 0.0),
        0.1,
        Vector3::new(0.0, 255.0, 205.0),
    ));
    scene.push(Sphere::new(
        Vector3::new(-0.3, 0.0, 0.0),
        0.15,
        Vector3::new(27.0, 44.0, 193.0),
    ));
    scene.push(Sphere::new(
        Vector3::new(0.5, 1.0, 0.0),
        0.2,
        Vector3::new(189.0, 44.0, 193.0),
    ));
    scene.push(Sphere::new(
        Vector3::new(0.5, 1.0, 0.0),
        0.2,
        Vector3::new(189.0, 44.0, 193.0),
    ));

    scene
}

fn draw_scene(canvas: &mut Canvas<Window>, cam: &Camera, scene: &Scene) {
    for j in 0..SCREEN_HEIGHT {
        for i in 0..SCREEN_WIDTH {
            //canvas.pixel(i as i16, j as i16, BACKGROUND_COLOR);

            let mut rng = rand::thread_rng();
            let x = (i as f32 + rng.gen::<f32>()) / SCREEN_WIDTH as f32;
            let y = (j as f32 + rng.gen::<f32>()) / SCREEN_HEIGHT as f32;
            let ray = cam.get_ray(x, y);

            let res = nearest_intersected_object(&scene, &ray);

            let (intersected_sphere, _distance) = match res {
                Some((sphere, distance)) => {
                    println!("Closest intersected object exists.");
                    (sphere, distance)
                }
                None => {
                    println!("No intersections");
                    let _res = canvas.pixel(i as i16, j as i16, BACKGROUND_COLOR);
                    continue;
                }
            };

            let res = canvas.pixel(i as i16, j as i16, intersected_sphere.get_color());

            match res {
                Ok(_) => {}
                Err(e) => println!("{}", e),
            }
        }
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window("Raytracer by Wiktor Flis", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    //canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    //canvas.clear();

    // initialize scene:
    println!("Initializing scene...");

    let scene = initialize_scene();
    let look_from = Vector3::new(0.0, 0.0, 0.0);
    let look_at = scene[0].center(); // Vector3::new(1.0, 0.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        look_from,
        look_at,
        Vector3::new(0.0, 1.0, 0.0),
        90.0,
        SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32,
        aperture,
        focus_dist,
    );

    println!("Drawing scene");
    draw_scene(&mut canvas, &cam, &scene);
    println!("Scene drawed");
    canvas.present();

    let mut events = sdl_context.event_pump()?;

    let mut lastx = 0;
    let mut lasty = 0;

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        break 'main;
                    }
                }

                Event::MouseButtonDown { x, y, .. } => {
                    let color = pixels::Color::RGB(x as u8, y as u8, 255);
                    let _ = canvas.line(lastx, lasty, x as i16, y as i16, color);
                    lastx = x as i16;
                    lasty = y as i16;
                    println!("mouse btn down at ({},{})", x, y);
                    canvas.present();
                }

                _ => {}
            }
        }
    }

    Ok(())
}
