use nalgebra::Vector3;
use sdl2::pixels::Color;

pub enum LightType {
    Ambient,
    Positional,
}

pub trait Light: Sync {
    fn light_type(&self) -> LightType;
    fn intensity(&self) -> f32;
    fn center(&self) -> Vector3<f32>;
}

pub struct PositionalLight {
    pub center: Vector3<f32>,
    pub intensity: f32,
    pub color: Vector3<f32>,
    //ambient: Vector3,
    //diffuse: Vector3,
    //specular: Vector3
}

impl PositionalLight {
    pub fn new(center: Vector3<f32>, intensity: f32, color: Vector3<f32>) -> Self {
        PositionalLight {
            center,
            intensity,
            color,
        }
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

impl Light for PositionalLight {
    fn light_type(&self) -> LightType {
        LightType::Positional
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }

    fn center(&self) -> Vector3<f32> {
        self.center
    }
}

pub struct AmbientLight {
    pub intensity: f32,
    pub color: Vector3<f32>,
    //ambient: Vector3,
    //diffuse: Vector3,
    //specular: Vector3,
}

impl AmbientLight {
    pub fn new(intensity: f32, color: Vector3<f32>) -> Self {
        AmbientLight { intensity, color }
    }

    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    pub fn light_type(&self) -> &str {
        "ambient"
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

impl Light for AmbientLight {
    fn light_type(&self) -> LightType {
        LightType::Ambient
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }

    fn center(&self) -> Vector3<f32> {
        Vector3::new(0.0, 0.0, 0.0)
    }
}
