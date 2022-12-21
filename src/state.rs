use crate::input::Input;

const MEM_SIZE: usize = 4096;
const STACK_SIZE: usize = 1024;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;


/// CHIP-8 Registers
struct Registers {
    /// The 16 8-bit general-purpose registers
    v: [u8; 16],

    /// The 8-bit delay timer
    /// Decrements at 60hz
    dt: u8,

    /// The 8-bit sound timer
    /// Decrements at 60hz
    st: u8,

    /// The 16-bit index register
    /// CHIP-8 addresses are only 12 bits wide, so only the lower 12 bits are used
    i: u16,

    /// The 16-bit program counter
    pc: u16,

    /// The stack pointer
    /// In this implementation, the stack pointer is 16 bits.
    sp: u16,

    /// The input register
    /// Holds state of each of the input buttons
    input: u16,
}

/// CHIP-8 Memory
pub struct Memory {
    /// The main memory
    /// The CHIP-8 has 4kB of RAM
    ram: [u8; MEM_SIZE],

    /// The stack
    /// Used mostly for addresses for subroutine calls.
    stack: [u16; STACK_SIZE],

    /// The display state
    /// For most modern implementations, the display is 64x32.
    vram: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}

pub struct Chip8 {
    registers: Registers,
    pub memory: Memory
}

impl Chip8 {
    /// Make a new Chip8
    pub fn new() -> Self {
        Self {
            registers: Registers {
                v: [0; 16],
                dt: 0,
                st: 0,
                i: 0,
                pc: 0,
                sp: 0,
                input: 0,
            },
            memory: Memory {
                ram: [0; MEM_SIZE],
                stack: [0; STACK_SIZE],
                vram: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            }
        }
    }

    /// Load a rom into memory
    pub fn load_rom(mut self, path: String) -> Self {

        // TODO implement

        self
    }

    /// Check if a tone is playing
    pub fn get_tone(&self) -> bool {
        self.registers.st > 0
    }

    /// Get the display state
    pub fn get_display(&self) -> &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT] {
        &self.memory.vram
    }

    /// Update the inputs
    pub fn change_input(&mut self, input: Input) {
        // TODO implement
    }
}