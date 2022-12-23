mod startup_systems;
mod systems;
mod args;

use crate::chip8::{Chip8, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::emulator::startup_systems::*;
use crate::emulator::systems::*;
use bevy::prelude::*;
use clap::Parser;
use crate::emulator::args::EmulatorArgs;

// display information
const PIXEL_SIZE: f32 = 10.0;
const ON_COLOR: Color = Color::RED;
const OFF_COLOR: Color = Color::BLACK;

const WINDOW_SIZE: (f32, f32) = (640.0, 320.0);

#[derive(Resource)]
pub struct Emulator {
    state: Chip8
}

#[derive(Component)]
pub struct Coordinate{
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
        .add_system(do_next_instruction)
        .add_system(update_display)
        .run();
}


