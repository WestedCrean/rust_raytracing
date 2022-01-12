extern crate sdl2;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::render::Canvas;
use sdl2::video::Window;

use bvh::ray::Ray;
use bvh::{Point3, Vector3};

mod intersections;
mod lights;
mod shapes;
mod utils;

pub use crate::intersections::{
    nearest_intersected_object, ray_sphere_intersection, NoIntersectionError,
};
pub use crate::lights::PositionalLight;
pub use crate::shapes::Sphere;
pub use crate::utils::vector_to_color;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const SCREEN_ASPECT_RATIO: f32 = SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32;

const FOV: u32 = 90;

const BACKGROUND_COLOR: pixels::Color = pixels::Color::RGB(243, 183, 127);

#[derive(Debug, Clone)]
struct DrawSceneError;

fn initialize() -> (Point3, Vec<Sphere>) {
    let camera_origin = Point3::new(0.0, 0.0, 0.0);
    let scene = initialize_scene();

    (camera_origin, scene)
}

fn initialize_scene() -> Vec<Sphere> {
    let mut scene = Vec::new();

    scene.push(Sphere {
        center: Point3::new(-0.2, 0.0, -1.0),
        radius: 0.7,
        color: Vector3::new(218.0, 255.0, 63.0),
    });

    scene.push(Sphere {
        center: Point3::new(400.0, 400.0, 51.0),
        radius: 5.0,
        color: Vector3::new(218.0, 255.0, 63.0),
    });

    scene.push(Sphere {
        center: Point3::new(0.1, -0.3, 0.0),
        radius: 0.1,
        color: Vector3::new(0.0, 255.0, 205.0),
    });
    scene.push(Sphere {
        center: Point3::new(-0.3, 0.0, 0.0),
        radius: 0.15,
        color: Vector3::new(27.0, 44.0, 193.0),
    });
    scene.push(Sphere {
        center: Point3::new(0.5, 1.0, 0.0),
        radius: 0.2,
        color: Vector3::new(189.0, 44.0, 193.0),
    });

    scene.push(Sphere {
        center: Point3::new(0.5, 1.0, 0.0),
        radius: 0.2,
        color: Vector3::new(189.0, 44.0, 193.0),
    });

    // 1 unit balls
    scene.push(Sphere {
        center: Point3::new(1.0, 0.0, 0.0),
        radius: 10.0,
        color: Vector3::new(200.0, 0.0, 0.0),
    });

    scene.push(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 10.0,
        color: Vector3::new(200.0, 0.0, 0.0),
    });

    scene.push(Sphere {
        center: Point3::new(-1.0, 0.0, 0.0),
        radius: 10.0,
        color: Vector3::new(200.0, 0.0, 0.0),
    });

    scene.push(Sphere {
        center: Point3::new(0.0, -1.0, 0.0),
        radius: 10.0,
        color: Vector3::new(200.0, 0.0, 0.0),
    });

    scene.push(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 10.0,
        color: Vector3::new(200.0, 0.0, 0.0),
    });

    scene.push(Sphere {
        center: Point3::new(0.0, 0.0, 1.0),
        radius: 10.0,
        color: Vector3::new(200.0, 0.0, 0.0),
    });

    scene
}

fn draw_scene(canvas: &mut Canvas<Window>, camera: Point3, scene: Vec<Sphere>) {
    let scale = (FOV as f32 * 0.5).to_radians().tan();
    for j in 0..SCREEN_HEIGHT {
        for i in 0..SCREEN_WIDTH {
            //canvas.pixel(i as i16, j as i16, BACKGROUND_COLOR);
            let x: f32 = 2.0 * (i as f32 + 0.5) / (SCREEN_WIDTH as f32 - 1.0) * scale;
            let y: f32 = 1.0
                - 2.0 * (j as f32 + 0.5)
                    / (SCREEN_HEIGHT as f32 * scale * 1.0)
                    / SCREEN_ASPECT_RATIO;
            //println!("x: {}, y: {}", x, y);
            let pixel = Point3::new(x, y, 0.0);
            let origin = camera;

            let direction = pixel - origin;

            let ray = Ray::new(origin, direction.normalize());

            let res = nearest_intersected_object(&scene, &ray);

            let (intersected_sphere, _distance) = match res {
                Some((sphere, distance)) => {
                    println!("Object hit");
                    (sphere, distance)
                }
                None => {
                    let _res = canvas.pixel(i as i16, j as i16, BACKGROUND_COLOR);
                    continue;
                }
            };

            let color = vector_to_color(intersected_sphere.color);
            let res = canvas.pixel(i as i16, j as i16, color);

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

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    // raytracing logic

    // initialize scene:
    println!("Initializing scene...");
    let (camera_origin, scene) = initialize();
    println!("Drawing scene");
    draw_scene(&mut canvas, camera_origin, scene);
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
