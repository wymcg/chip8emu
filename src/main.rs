extern crate core;

use crate::emulator::run_emulator;

mod chip8;
mod emulator;
mod input;
mod instructions;

fn main() {
    run_emulator();
}
