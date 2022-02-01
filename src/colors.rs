use nalgebra::Vector3;
use sdl2::pixels::Color;

pub const PARADISE_PINK: Color = Color::RGB(239, 71, 111);
pub const ORANGE_YELLOW_CRAYOLA: Color = Color::RGB(255, 209, 102);
pub const CARIBBEAN_GREEN: Color = Color::RGB(6, 214, 160);
pub const AQUAMARINE: Color = Color::RGB(76, 224, 179);
pub const CYCLAMEN: Color = Color::RGB(232, 106, 14);
pub const MIDDLE_YELLOW: Color = Color::RGB(247, 231, 51);
pub const PINK: Color = Color::RGB(247, 37, 133);
pub const BLUE: Color = Color::RGB(0, 150, 199);
pub const PURPLE: Color = Color::RGB(114, 9, 183);
pub const DEEP_PURPLE: Color = Color::RGB(72, 12, 168);
pub const SPACE: Color = Color::RGB(63, 55, 201);
pub const DEEP_MAGENTA: Color = Color::RGB(181, 23, 158);
pub const BLACK: Color = Color::RGB(0, 0, 32);
pub const WHITE: Color = Color::RGB(255, 255, 255);

pub fn get_color(vec: Vector3<f32>) -> Color {
    Color::RGB(vec[0] as u8, vec[1] as u8, vec[2] as u8)
}

pub fn get_vector(col: Color) -> Vector3<f32> {
    Vector3::new(col.r as f32, col.g as f32, col.b as f32)
}
