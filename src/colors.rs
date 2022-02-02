use nalgebra::Vector3;
use sdl2::pixels::Color;

pub const PARADISE_PINK: Color = Color::RGB(239, 71, 111);
pub const ORANGE_YELLOW_CRAYOLA: Color = Color::RGB(255, 209, 102);
pub const CARIBBEAN_GREEN: Color = Color::RGB(6, 214, 160);
pub const AQUAMARINE: Color = Color::RGB(76, 224, 179);
pub const MEDIUM_SPRING_GREEN: Color = Color::RGB(89, 255, 160);
pub const METALLIC_SEAWEED: Color = Color::RGB(54, 132, 145);
pub const TRUE_BLUE: Color = Color::RGB(4, 102, 200);
pub const MIDDLE_YELLOW: Color = Color::RGB(247, 231, 51);
pub const PINK: Color = Color::RGB(247, 37, 133);
pub const BLUE: Color = Color::RGB(0, 150, 199);
pub const NEON_BLUE: Color = Color::RGB(105, 112, 252);
pub const DEEP_PURPLE: Color = Color::RGB(72, 12, 168);
pub const SPACE: Color = Color::RGB(63, 55, 201);
pub const SILVER: Color = Color::RGB(235, 235, 235);
pub const ROSSO_CORSA: Color = Color::RGB(208, 0, 0);
pub const RUST: Color = Color::RGB(177, 79, 48);
pub const ORANGE_YELLOW: Color = Color::RGB(238, 185, 2);
pub const BLACK: Color = Color::RGB(0, 0, 32);
pub const WHITE: Color = Color::RGB(255, 255, 255);

pub fn get_color(vec: Vector3<f32>) -> Color {
    Color::RGB(vec[0] as u8, vec[1] as u8, vec[2] as u8)
}

pub fn get_vector(col: Color) -> Vector3<f32> {
    Vector3::new(col.r as f32, col.g as f32, col.b as f32)
}
