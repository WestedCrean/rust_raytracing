use bvh::Vector3;
use sdl2::pixels::Color;

pub fn vector_to_color(color: Vector3) -> Color {
    let [r, g, b] = color.as_ivec3().to_array();
    Color::RGB(r as u8, g as u8, b as u8)
}
