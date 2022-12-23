mod args;
mod startup_systems;
mod systems;
mod util;

use crate::chip8::Chip8;
use crate::emulator::args::EmulatorArgs;
use crate::emulator::startup_systems::*;
use crate::emulator::systems::*;
use bevy::prelude::KeyCode::*;
use bevy::prelude::*;
use clap::Parser;

// color information
const ON_COLOR: Color = Color::RED;
const OFF_COLOR: Color = Color::BLACK;

const KEYMAP: [(KeyCode, u8); 16] = [
    (Key1, 0x1),
    (Key2, 0x2),
    (Key3, 0x3),
    (Key4, 0xC),
    (Q, 0x4),
    (W, 0x5),
    (E, 0x6),
    (R, 0xD),
    (A, 0x7),
    (S, 0x8),
    (D, 0x9),
    (F, 0xE),
    (Z, 0xA),
    (X, 0x0),
    (C, 0xB),
    (V, 0xF),
];

const WINDOW_SIZE: (f32, f32) = (640.0, 320.0);

#[derive(Resource)]
pub struct Emulator {
    state: Chip8,
}

#[derive(Component)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

pub fn run_emulator() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WINDOW_SIZE.0,
                height: WINDOW_SIZE.1,
                title: "CHIP-8".to_string(),
                resizable: true,
                decorations: true,
                cursor_visible: true,
                mode: WindowMode::Windowed,
                ..default()
            },
            ..default()
        }))
        .insert_resource(EmulatorArgs::parse())
        .add_startup_system(emu_setup)
        .add_startup_system(camera_setup)
        .add_startup_system(pixels_setup)
        .add_system(get_input)
        .add_system(do_next_instruction)
        .add_system(update_display)
        .add_system(window_resize_pixel)
        .add_system(window_resize_camera)
        .run();
}
