extern crate core;

use crate::state::{Chip8, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use bevy::prelude::*;

mod input;
mod instructions;
mod state;

const ROM_PATH: &str = "./roms/test_opcode.ch8";
const FONT_PATH: &str = "./fonts/default_font.ch8";

// display information
const PIXEL_SIZE: f32 = 10.0;
const ON_COLOR: Color = Color::RED;
const OFF_COLOR: Color = Color::BLACK;

#[derive(Component)]
struct Emulator {
    state: Chip8
}

#[derive(Component)]
struct Coordinate{
    x: usize,
    y: usize,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: DISPLAY_WIDTH as f32 * PIXEL_SIZE,
                height: DISPLAY_HEIGHT as f32 * PIXEL_SIZE,
                title: "CHIP-8".to_string(),
                resizable: false,
                decorations: true,
                cursor_visible: true,
                mode: WindowMode::Windowed,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(emu_setup)
        .add_startup_system(camera_setup)
        .add_startup_system(pixels_setup)
        .add_system(do_next_instruction)
        .add_system(update_display)
        .run();
}

/// Make the camera
fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(
                (DISPLAY_WIDTH as f32 * PIXEL_SIZE / 2.0) - (PIXEL_SIZE / 2.0),
                (DISPLAY_HEIGHT as f32 * PIXEL_SIZE / 2.0) + (PIXEL_SIZE / 2.0),
                0.0
            ),
            ..default()
        },
        ..default()
    });
}

/// Make the emulator
fn emu_setup(mut commands: Commands) {
    commands.spawn(Emulator {
        state: Chip8::new().load_font(FONT_PATH).load_rom(ROM_PATH)
    });
}

/// Make all the pixels
fn pixels_setup(mut commands: Commands) {
    // make the pixels
    for x in 0..DISPLAY_WIDTH {
        for y in 0..DISPLAY_HEIGHT {
            commands.spawn((Coordinate {x, y}, SpriteBundle {
                sprite: Sprite {
                    color: OFF_COLOR,
                    custom_size: Some(Vec2::new(PIXEL_SIZE, PIXEL_SIZE)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        x as f32 * PIXEL_SIZE,
                        (DISPLAY_HEIGHT as f32 * PIXEL_SIZE) - (y as f32 * PIXEL_SIZE),
                        0.0
                    ),
                    ..default()
                },
                ..default()
            }));
        }
    }
}

/// Update the display based on the emulator state
fn update_display(mut pixels_query: Query<(&mut Coordinate, &mut Sprite)>, emu_query: Query<&Emulator>) {

    for emulator in &emu_query {
        // update the pixels with the state
        for (coord, mut pixel) in pixels_query.iter_mut() {
            if emulator.state.get_display()[coord.y][coord.x] {
                pixel.color = ON_COLOR;
            } else {
                pixel.color = OFF_COLOR;
            }

        }
    }
}

/// Do the next instruction
fn do_next_instruction(mut query: Query<&mut Emulator>) {
    for mut emulator in query.iter_mut() {
        match emulator.state.do_next_instruction() {
            Ok(_) => {/* do nothing */}
            Err(op) => {panic!("Invalid opcode {:#06x}", op)}
        };
    }
}