use bevy::prelude::*;
use crate::emulator::{Coordinate, Emulator, OFF_COLOR, ON_COLOR};

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
        Ok(_) => {/* do nothing */}
        Err(op) => {panic!("Invalid opcode {:#06x}", op)}
    };
}
