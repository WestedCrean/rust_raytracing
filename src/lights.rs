use nalgebra::Vector3;
use sdl2::pixels::Color;

pub struct PositionalLight {
    pub center: Vector3<f32>,
    pub color: Vector3<f32>,
    //ambient: Vector3,
    //diffuse: Vector3,
    //specular: Vector3,
}

impl PositionalLight {
    pub fn new(center: Vector3<f32>, color: Vector3<f32>) -> Self {
        PositionalLight { center, color }
    }

    pub fn center(&self) -> Vector3<f32> {
        self.center
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
