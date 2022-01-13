use nalgebra::Vector3;
use sdl2::pixels::Color;
use crate::intersections::

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub color: Vector3<f32>,
    //shininess: f32,
    //ambient: Vector3<f32>,
    //diffuse: Vector3<f32>,
    //specular: Vector3<f32>,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, color: Vector3<f32>) -> Self {
        Sphere {
            center,
            radius,
            color,
        }
    }

    pub fn center(&self) -> Vector3<f32> {
        self.center
    }
    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn color_vector(&self) -> Vector3<f32> {
        self.color
    }

    pub fn get_color(&self) -> Color {
        let r = self.color[0];
        let g = self.color[1];
        let b = self.color[2];
        Color::RGB(r as u8, g as u8, b as u8)
    }
}
