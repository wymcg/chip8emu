extern crate core;

use crate::emulator::run_emulator;


mod input;
mod instructions;
mod chip8;
mod emulator;



fn main() {
    run_emulator();
}
