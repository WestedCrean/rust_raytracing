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

pub fn get_color(vec: Vector3<f32>) -> Color {
    let r = vec[0];
    let g = vec[1];
    let b = vec[2];
    Color::RGB(r as u8, g as u8, b as u8)
}

pub fn get_vector(col: Color) -> Vector3<f32> {
    let r = col.r;
    let g = col.g;
    let b = col.g;
    Vector3::new(r as f32, g as f32, b as f32)
}
