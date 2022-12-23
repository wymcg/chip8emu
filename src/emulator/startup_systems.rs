use crate::chip8::{Chip8, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::emulator::args::EmulatorArgs;
use crate::emulator::{Coordinate, Emulator, OFF_COLOR};
use bevy::prelude::*;
use crate::emulator::util::{get_camera_translation, get_pixel_translation};

/// Make the camera
pub fn camera_setup(mut commands: Commands, windows: Res<Windows>) {
    let window = windows
        .get_primary()
        .expect("Unable to get primary window!");

    let pixel_size: (f32, f32) = (
        window.width() / DISPLAY_WIDTH as f32,
        window.height() / DISPLAY_HEIGHT as f32,
    );

    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: get_camera_translation(pixel_size),
            ..default()
        },
        ..default()
    });
}

/// Make the emulator
pub fn emu_setup(mut commands: Commands, args: Res<EmulatorArgs>) {
    commands.insert_resource(Emulator {
        state: Chip8::new()
            .load_font(args.font.clone())
            .load_rom(args.rom.clone()),
    });
}

/// Make all the pixels
pub fn pixels_setup(mut commands: Commands, windows: Res<Windows>) {
    let window = windows
        .get_primary()
        .expect("Unable to get primary window!");

    let pixel_size: (f32, f32) = (
        window.width() / DISPLAY_WIDTH as f32,
        window.height() / DISPLAY_HEIGHT as f32,
    );

    // make the pixels
    for x in 0..DISPLAY_WIDTH {
        for y in 0..DISPLAY_HEIGHT {
            commands.spawn((
                Coordinate { x, y },
                SpriteBundle {
                    sprite: Sprite {
                        color: OFF_COLOR,
                        custom_size: Some(Vec2::new(pixel_size.0, pixel_size.1)),
                        ..default()
                    },
                    transform: Transform {
                        translation: get_pixel_translation(x, y, pixel_size),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    }
}
