mod camera;
mod colors;
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

use crate::intersections::nearest_intersected_object;
use crate::intersections::IntersectionRecord;
use rand::Rng;
use rayon::prelude::*;
// use crate::lights::PositionalLight;
use crate::camera::Camera;
use crate::colors::{
    get_color, get_vector, BLACK, CARIBBEAN_GREEN, CYCLAMEN, DEEP_PURPLE, MIDDLE_YELLOW,
    ORANGE_YELLOW_CRAYOLA, PARADISE_PINK,
};
use crate::scene::Scene;
use crate::shapes::Sphere;
use nalgebra::Vector3;
use sdl2::pixels::Color;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const SAMPLES_PER_PIXEL: u32 = 4;

// colors palette

const BACKGROUND_COLOR: pixels::Color = BLACK;

#[derive(Debug, Clone)]
struct DrawSceneError;

fn initialize_scene() -> Scene {
    let mut scene = Scene::default();

    scene.push(Sphere::new(
        Vector3::new(2.0, 0.0, 0.0),
        0.7,
        get_vector(CARIBBEAN_GREEN),
    ));

    scene.push(Sphere::new(
        Vector3::new(0.96, 0.36, 0.0),
        0.1,
        get_vector(CYCLAMEN),
    ));

    scene.push(Sphere::new(
        Vector3::new(0.96, 0.85, -0.52),
        0.15,
        get_vector(DEEP_PURPLE),
    ));

    scene.push(Sphere::new(
        Vector3::new(0.96, -0.53, -0.36),
        0.15,
        get_vector(ORANGE_YELLOW_CRAYOLA),
    ));

    scene.push(Sphere::new(
        Vector3::new(1.0, -0.9, 0.1),
        0.23,
        get_vector(PARADISE_PINK),
    ));

    scene.push(Sphere::new(
        Vector3::new(1.3, 0.6, 0.6),
        0.15,
        get_vector(MIDDLE_YELLOW),
    ));

    scene.push(Sphere::new(
        Vector3::new(3.0, -0.4, 1.0),
        0.2,
        get_vector(DEEP_PURPLE),
    ));

    scene
}

fn draw_scene(canvas: &mut Canvas<Window>, cam: &Camera, scene: &Scene) -> Result<(), String> {
    // using nice and fast rayon code used from https://github.com/fralken/ray-tracing-in-one-weekend/blob/master/src/main.rs
    // courtesy of https://github.com/fralken
    // as I don't understand flat maps and rayon very much yet
    let colors = (0..SCREEN_HEIGHT)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            (0..SCREEN_WIDTH)
                .flat_map(|i| {
                    let color: Vector3<f32> = (0..SAMPLES_PER_PIXEL)
                        .map(|_| {
                            let x = (i as f32) / SCREEN_WIDTH as f32;
                            let y = (j as f32) / SCREEN_HEIGHT as f32;

                            let ray = cam.get_ray(x, y);

                            let res = nearest_intersected_object(scene, &ray, 0.001, f32::MAX);

                            match res {
                                Some(res) => return get_vector(res.object_color),
                                None => return get_vector(BACKGROUND_COLOR),
                            }
                        })
                        .sum();

                    color
                        .iter()
                        .map(|c| (c / SAMPLES_PER_PIXEL as f32) as u8)
                        .collect::<Vec<u8>>()
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    for j in 0..SCREEN_HEIGHT {
        for i in 0..SCREEN_WIDTH {
            canvas.pixel(
                i as i16,
                j as i16,
                Color::RGB(
                    colors[(3 * j * SCREEN_WIDTH + i * 3 + 0) as usize],
                    colors[(3 * j * SCREEN_WIDTH + i * 3 + 1) as usize],
                    colors[(3 * j * SCREEN_WIDTH + i * 3 + 2) as usize],
                ),
            )?;
        }
    }

    Ok(())
}

fn render_scene(canvas: &mut Canvas<Window>, look_at_object: i32) {
    // initialize scene:
    println!("Initializing scene...");

    let scene = initialize_scene();
    let look_from = Vector3::new(-0.5, 0.0, 0.0);
    let look_at = scene.get_nth_element_center(look_at_object);
    let focus_dist = 10.0;
    let aperture = 0.1;

    let look_at = match look_at {
        Some(look_at) => look_at,
        None => Vector3::new(1.0, 0.0, 0.0),
    };

    let cam = Camera::new(
        look_from,
        look_at,
        Vector3::new(0.0, 1.0, 0.0),
        55.0,
        SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32,
        aperture,
        focus_dist,
    );

    println!("Drawing scene");
    draw_scene(canvas, &cam, &scene);
    println!("Scene drawed");
    canvas.present();
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
    let mut look_at_object = 0;

    render_scene(&mut canvas, look_at_object);

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

                    if keycode == Keycode::Space {
                        look_at_object += 1;
                        render_scene(&mut canvas, look_at_object);
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
