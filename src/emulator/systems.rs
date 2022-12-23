use crate::emulator::{Coordinate, Emulator, OFF_COLOR, ON_COLOR};
use bevy::prelude::*;
use bevy::window::WindowResized;
use crate::chip8::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::emulator::util::{get_camera_translation, get_pixel_translation};

/// Update the display based on the emulator state
pub fn update_display(mut pixels_query: Query<(&mut Coordinate, &mut Sprite)>, emu: Res<Emulator>) {
    // update the pixels with the state
    for (coord, mut pixel) in pixels_query.iter_mut() {
        if emu.state.get_display()[coord.y][coord.x] {
            pixel.color = ON_COLOR;
        } else {
            pixel.color = OFF_COLOR;
        }
    }
}

/// Do the next instruction
pub fn do_next_instruction(mut emu: ResMut<Emulator>) {
    match emu.state.do_next_instruction() {
        Ok(_) => { /* do nothing */ }
        Err(op) => {
            panic!("Invalid opcode {:#06x}", op)
        }
    };
}

/// Manage pixels upon window resizing
pub fn window_resize_pixel(
    mut events: EventReader<WindowResized>,
    mut pixels: Query<(&mut Coordinate, &mut Sprite, &mut Transform)>,
) {
    for event in events.iter() {

        // get the size of a pixel
        let pixel_size: (f32, f32) = (
            event.width / DISPLAY_WIDTH as f32,
            event.height / DISPLAY_HEIGHT as f32,
        );

        // change the size and translation of each pixel
        for (coord, mut pixel, mut transform) in &mut pixels {
            // change the location of the pixel
            transform.translation = get_pixel_translation(coord.x, coord.y, pixel_size);

            // change the size of the pixel
            pixel.custom_size = Some(Vec2::new(pixel_size.0, pixel_size.1));
        }
    }
}
/// Manage camera upon window resizing
pub fn window_resize_camera(
    mut events: EventReader<WindowResized>,
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    for event in events.iter() {

        // get the size of a pixel
        let pixel_size: (f32, f32) = (
            event.width / DISPLAY_WIDTH as f32,
            event.height / DISPLAY_HEIGHT as f32,
        );

        // change the camera translation
        for mut camera in &mut cameras {
            camera.translation = get_camera_translation(pixel_size);
        }

    }
}
