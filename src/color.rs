use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color) -> String {
    format!("{} {} {}",
        ((255.999 * pixel_color.x()) as u64),
        ((255.999 * pixel_color.y()) as u64),
        ((255.999 * pixel_color.z()) as u64)
    )
}
