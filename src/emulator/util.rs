use bevy::math::Vec3;
use crate::chip8::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

/// Get the camera translation
pub fn get_camera_translation(pixel_size: (f32, f32)) -> Vec3 {
    Vec3::new(
        (DISPLAY_WIDTH as f32 * pixel_size.0 as f32 / 2.0) - (pixel_size.0 as f32 / 2.0),
        (DISPLAY_HEIGHT as f32 * pixel_size.1 as f32 / 2.0) + (pixel_size.1 as f32 / 2.0),
        0.0,
    )
}

/// Get the translation for a certain pixel
pub fn get_pixel_translation(coord_x: usize, coord_y: usize, pixel_size: (f32, f32)) -> Vec3 {
    Vec3::new(
        coord_x as f32 * pixel_size.0,
        (DISPLAY_HEIGHT as f32 * pixel_size.1 as f32)
            - (coord_y as f32 * pixel_size.1 as f32),
        0.0,
    )
}