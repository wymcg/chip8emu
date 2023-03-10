use crate::input::Input;
use crate::instructions::Instruction::*;
use crate::instructions::{Immediate, Instruction, Register};
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{BufReader, Read};

const MEM_SIZE: usize = 4096;
const STACK_SIZE: usize = 1024;
const PROGMEM_START: u16 = 0x200;
const FONTMEM_START: u16 = 0x000;

const DEFAULT_FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

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
    i: usize,

    /// The 16-bit program counter
    pc: usize,

    /// The stack pointer
    /// In this implementation, the stack pointer is 16 bits.
    sp: usize,
}

/// CHIP-8 Memory
pub struct Memory {
    /// The main memory
    /// The CHIP-8 has 4kB of RAM
    ram: [u8; MEM_SIZE],

    /// The stack
    /// Used mostly for addresses for subroutine calls.
    stack: [usize; STACK_SIZE],

    /// The display state
    /// For most modern implementations, the display is 64x32.
    vram: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}

pub struct InputState {
    curr: u16,
    prev: u16,
    key_just_released: bool,
}

pub struct Chip8 {
    /// The registers of the CHIP-8
    registers: Registers,

    /// Memory such as RAM, the stack, and VRAM
    pub memory: Memory,

    /// The current inputs, and the previous state of the input at the last cycle
    input: InputState,
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
                pc: 0x200,
                sp: 0,
            },
            memory: Memory {
                ram: [0; MEM_SIZE],
                stack: [0; STACK_SIZE],
                vram: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            },
            input: InputState {
                curr: 0b0000_0000_0000_0000,
                prev: 0b0000_0000_0000_0000,
                key_just_released: false,
            },
        }
    }

    /// Load a rom into memory
    pub fn load_rom(mut self, path: String) -> Self {
        // open the file
        let file: File = File::open(path).expect("Unable to open ROM file!");

        // make the vec to hold the bytes
        let mut bytes: Vec<u8> = Vec::new();

        // read the file into the bytes vector
        BufReader::new(file)
            .read_to_end(&mut bytes)
            .expect("Unable to read file!");

        for i in 0..bytes.len() {
            self.memory.ram[PROGMEM_START as usize + i] = bytes[i];
        }

        self
    }

    pub fn load_font(mut self, path: Option<String>) -> Self {
        // make the vec to hold the bytes
        let mut bytes: Vec<u8> = Vec::new();

        match path {
            None => {
                bytes = DEFAULT_FONT.to_vec();
            }
            Some(path) => {
                // open the file
                let file: File = File::open(path).expect("Unable to open font file!");

                // read the file into the bytes vector
                BufReader::new(file)
                    .read_to_end(&mut bytes)
                    .expect("Unable to read file!");
            }
        }

        // load the font into memory
        for i in 0..bytes.len() {
            self.memory.ram[FONTMEM_START as usize + i] = bytes[i];
        }

        self
    }

    /// Check if a tone is playing
    pub fn get_tone(&self) -> bool {
        self.registers.st > 0
    }

    /// Get the display state
    /// It is assumed that this is called 60 times a second
    pub fn do_frame(&mut self) -> &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT] {
        // decrement ST if needed
        if self.registers.st > 0 {
            self.registers.st -= 1;
        }

        // decrement DT if needed
        if self.registers.dt > 0 {
            self.registers.dt -= 1;
        }

        &self.memory.vram
    }

    /// Update the inputs
    pub fn change_input(&mut self, input: Input) {
        // set the previous input
        self.input.prev = self.input.curr;

        // update the current input
        match input {
            Input::Pressed(key) => {
                self.input.curr |= 0x1 << key; // set the n-th bit to 1
            }
            Input::Unpressed(key) => {
                self.input.curr &= !(0x1 << key); // set the n-th bit to 0
                self.input.key_just_released = true;
            }
        }
    }

    /// Do the next instruction and return the result, containing the opcode that was just dealt with
    /// This should be called about 500 times a second
    /// Or, a little under 9 times per call to do_frame()
    pub fn do_next_instruction(&mut self) -> Result<u16, u16> {
        // get the current opcode for returning results
        let current_opcode: u16 = self.get_current_opcode();

        match self.get_current_instruction() {
            Sys(_) => { /* intentionally ignore */ }
            Cls => {
                // clear vram
                // set all spaces in vram to false
                for y in 0..self.memory.vram.len() {
                    for x in 0..self.memory.vram[y].len() {
                        self.memory.vram[y][x] = false;
                    }
                }
            }
            Ret => {
                // return from a subroutine
                // decrement the stack pointer
                self.registers.sp -= 1;

                // set the program counter to be the newly popped address
                self.registers.pc = self.memory.stack[self.registers.sp];
            }
            Jump(addr) => {
                // jump to the given address
                // set the program counter to be the given address
                self.registers.pc = addr - 0x2;
            }
            JumpWithOffset(addr) => {
                // jump to the given address, offset by the value in V0
                self.registers.pc = addr + self.registers.v[0x0] as usize;
            }
            Call(addr) => {
                // call subroutine at the given address
                // put the current PC at the top of the stack
                self.memory.stack[self.registers.sp] = self.registers.pc;
                self.registers.sp += 1;

                // replace the current PC with the given address
                self.registers.pc = addr - 0x2;
            }
            SkipEqualImm(reg, imm) => {
                // if the contents of the given register is equal to the immediate,
                // point the PC past the next instruction
                if self.registers.v[reg] == imm {
                    self.registers.pc += 2;
                }
            }
            SkipNotEqualImm(reg, imm) => {
                // if the contents of the given register is not equal to the immediate,
                // point the PC past the next instruction
                if self.registers.v[reg] != imm {
                    self.registers.pc += 2;
                }
            }
            SkipEqualReg(regx, regy) => {
                // if the contents of the two registers are the same,
                // point the PC past the next instruction
                if self.registers.v[regx] == self.registers.v[regy] {
                    self.registers.pc += 2;
                }
            }
            SkipNotEqualReg(regx, regy) => {
                // if th contents of the two registers are not the same,
                // point the PC past the next instruction
                if self.registers.v[regx] != self.registers.v[regy] {
                    self.registers.pc += 2;
                }
            }
            LoadImm(reg, imm) => {
                // load an immediate value into a register
                self.registers.v[reg] = imm;
            }
            LoadReg(regx, regy) => {
                // load the contents of one register into another
                self.registers.v[regx] = self.registers.v[regy];
            }
            LoadAddress(addr) => {
                // load the index register with the given address
                self.registers.i = addr;
            }
            ReadDelayTimer(reg) => {
                // read the delay timer into a register
                self.registers.v[reg] = self.registers.dt;
            }
            WriteDelayTimer(reg) => {
                // write the delay timer with the contents of a register
                self.registers.dt = self.registers.v[reg];
            }
            WriteSoundTimer(reg) => {
                // write the sound timer with the contents of a register
                self.registers.st = self.registers.v[reg];
            }
            AddImm(reg, imm) => {
                // get the result
                self.registers.v[reg] = self.registers.v[reg].wrapping_add(imm);
            }
            AddReg(regx, regy) => {
                // add together VX and VY and put the result in VX
                // get the result
                let (result, overflow): (u8, bool) = self.registers.v[regx]
                    .overflowing_add(self.registers.v[regy]);

                // check if was an overflow and set the VF register if so
                if overflow {
                    self.registers.v[0xF] = 0x1
                } else {
                    self.registers.v[0xF] = 0x0;
                }

                // set VX with the result
                self.registers.v[regx] = result;
            }
            AddIndex(reg) => {
                // add I to VX and store in I
                self.registers.i = self
                    .registers
                    .i
                    .wrapping_add(self.registers.v[reg] as usize);
            }
            SubReg(regx, regy) => {
                // subtract VY from VX and put the result in VX
                // get the result
                let (result, borrow): (u8, bool) = self.registers.v[regx]
                    .overflowing_sub(self.registers.v[regy]);

                // set VF depending on whether not there was a borrow
                if borrow {
                    self.registers.v[0xF] = 0x0;
                } else {
                    self.registers.v[0xF] = 0x1;
                }

                // store the result in VX
                self.registers.v[regx] = result;
            }
            SubNReg(regx, regy) => {
                // subtract VX from VY and put the result in VY
                // get the result
                let (result, borrow): (u8, bool) = self.registers.v[regy]
                    .overflowing_sub(self.registers.v[regx]);

                // set VF depending on whether or not there was a borrow
                if borrow {
                    self.registers.v[0xF] = 0x0;
                } else {
                    self.registers.v[0xF] = 0x1;
                }

                // store the result in VX
                self.registers.v[regx] = result;
            }
            ShiftRightReg(regx, regy) => {
                // put the value of VY into VX
                self.registers.v[regx] = self.registers.v[regy];

                // get the lsb
                let lsb = self.registers.v[regx] & 0x01;

                // shift the register right 1
                self.registers.v[regx] >>= 1;

                // set VF with the lsb
                self.registers.v[0xF] = lsb;
            }
            ShiftLeftReg(regx, regy) => {
                // put the value of VY into VX
                self.registers.v[regx] = self.registers.v[regy];

                // get the msb
                let msb = (self.registers.v[regx] & 0x80) >> 7;

                // shift the register left one
                self.registers.v[regx] <<= 1;

                // set VF with the msb
                self.registers.v[0xF] = msb;
            }
            OrReg(regx, regy) => {
                // or together VX and VY and put the result in VX
                self.registers.v[regx] |= self.registers.v[regy];

                // reset the VF flag
                self.registers.v[0xF] = 0x00;
            }
            AndReg(regx, regy) => {
                // and together VX and VY and put the result in VX
                self.registers.v[regx] &= self.registers.v[regy];

                // reset the VF flag
                self.registers.v[0xF] = 0x00;
            }
            XorReg(regx, regy) => {
                // xor together VX and VY and put the result in VX
                self.registers.v[regx] ^= self.registers.v[regy];

                // reset the VF flag
                self.registers.v[0xF] = 0x00;
            }
            RandAndImmediate(reg, imm) => {
                // generate a random value, and with imm, and store in VX
                self.registers.v[reg] = thread_rng().gen::<u8>() & imm;

                // reset the VF flag
                self.registers.v[0xF] = 0x00;
            }
            Draw(regx, regy, imm) => {
                // reset VF
                self.registers.v[0xF] = 0x0;

                // get x and y to start drawing the sprite
                let start_x: usize = self.registers.v[regx] as usize % DISPLAY_WIDTH;
                let start_y: usize = self.registers.v[regy] as usize % DISPLAY_HEIGHT;

                for row in 0..imm as usize {
                    for col in 0..8 as usize {
                        // get this pixel in the sprite
                        let pixel_state: bool = (self.memory.ram
                            [self.registers.i + row]
                            & (0x1 << (7 - col)))
                            > 0;

                        // only attempt to change this sprite if this bit is set
                        if pixel_state {
                            // get the x and y for this pixel
                            let x = start_x + col;
                            let y = start_y + row;

                            // do not draw this pixel if it goes off the side of the screen
                            if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
                                continue;
                            }

                            // set the collision flag if this coord is already set
                            if self.memory.vram[y][x] {
                                self.registers.v[0xF] = 0x1;
                            }

                            // write vram
                            self.memory.vram[y][x] ^= pixel_state;
                        }
                    }
                }
            }
            SetSpriteLoc(reg) => {
                // set I with the sprite info for the character in reg
                self.registers.i = self.registers.v[reg] as usize * 0x05;
                // each sprite is 5 bytes long
            }
            SkipIfKeyPressed(reg) => {
                // skip the next instruction if the input specified in the register is pressed
                if self.input.curr & (0x1 << self.registers.v[reg]) > 0 {
                    self.registers.pc += 2;
                }
            }
            SkipIfKeyNotPressed(reg) => {
                // skip the next instruction if the input specified in the register is not pressed
                if self.input.curr & (0x1 << self.registers.v[reg]) == 0 {
                    self.registers.pc += 2;
                }
            }
            StoreBCD(reg) => {
                // store BCD representation of VX in I, I+1, and I+2
                // get the hundreds, tens, and ones places
                let hundreds: u8 = self.registers.v[reg] / 100;
                let tens: u8 = (self.registers.v[reg] % 100) / 10;
                let ones: u8 = self.registers.v[reg] % 10;

                self.memory.ram[self.registers.i] = hundreds;
                self.memory.ram[self.registers.i + 1] = tens;
                self.memory.ram[self.registers.i + 2] = ones;
            }
            StoreRegisters(reg) => {
                // store registers V0-VX in memory starting at I
                for r in 0..=reg as usize {
                    self.memory.ram[self.registers.i + r] =
                        self.registers.v[r];
                }

                // increment I
                self.registers.i += reg + 1;
            }
            ReadRegisters(reg) => {
                // populate registers V0-VX with data starting from I
                for r in 0..=reg as usize {
                    self.registers.v[r] =
                        self.memory.ram[self.registers.i + r];
                }

                // increment I
                self.registers.i += reg + 1;
            }
            StoreKeypress(reg) => {
                // only store the keypress once it is released
                if self.input.key_just_released {
                    // get the inputs that have changed
                    let changed_inputs: u16 = self.input.curr ^ self.input.prev;

                    // get the inputs that have just been released
                    let released_inputs: u16 = changed_inputs & !self.input.curr;

                    // set the given input to be the greatest input that was just released
                    self.registers.v[reg] = (released_inputs as f32).log2() as u8;

                } else {
                    // counteract the PC increment that comes later
                    self.registers.pc -= 2;
                }

            }
            _ => {
                return Err(current_opcode);
            }
        }

        // point the PC to the next instruction
        self.registers.pc += 2;

        // reset the key release flag
        self.input.key_just_released = false;

        Ok(current_opcode)
    }

    /// Get the opcode at the PC
    fn get_current_opcode(&self) -> u16 {
        ((self.memory.ram[self.registers.pc] as u16) << 8)
            | (self.memory.ram[self.registers.pc + 1] as u16)
    }

    /// Identify the instruction at the PC
    fn get_current_instruction(&self) -> Instruction {
        // get the full opcode
        let opcode: u16 = self.get_current_opcode();

        // get the opcode components
        let inst_word: u8 = ((opcode & 0xF000) >> 12) as u8;
        let addr: usize = (opcode & 0x0FFF) as usize;
        let nibble: u8 = (opcode & 0x000F) as u8;
        let imm: Immediate = (opcode & 0x00FF) as u8;
        let regx: Register = ((opcode & 0x0F00) >> 8) as usize;
        let regy: Register = ((opcode & 0x00F0) >> 4) as usize;

        // use the components to make the instruction to return
        match inst_word {
            0x0 => {
                // SYS, CLS, or RET instruction
                match addr {
                    0x0E0 => Cls,
                    0x0EE => Ret,
                    _ => Sys(addr),
                }
            }
            0x1 => {
                // JP instruction
                Jump(addr)
            }
            0x2 => {
                // CALL instruction
                Call(addr)
            }
            0x3 => {
                // SE instruction (immediate)
                SkipEqualImm(regx, imm)
            }
            0x4 => {
                // SNE instruction (immediate)
                SkipNotEqualImm(regx, imm)
            }
            0x5 => {
                // SE instruction (register)
                SkipEqualReg(regx, regy)
            }
            0x6 => {
                // LD instruction (immediate)
                LoadImm(regx, imm)
            }
            0x7 => {
                // ADD instruction (immediate)
                AddImm(regx, imm)
            }
            0x8 => {
                // LD, OR, AND, XOR, ADD, SUB, SUBN, SHR, and SHL instructions for registers
                match nibble {
                    0x0 => LoadReg(regx, regy),
                    0x1 => OrReg(regx, regy),
                    0x2 => AndReg(regx, regy),
                    0x3 => XorReg(regx, regy),
                    0x4 => AddReg(regx, regy),
                    0x5 => SubReg(regx, regy),
                    0x6 => ShiftRightReg(regx, regy),
                    0x7 => SubNReg(regx, regy),
                    0xE => ShiftLeftReg(regx, regy),
                    _ => Unknown,
                }
            }
            0x9 => {
                // SNE instruction (register)
                SkipNotEqualReg(regx, regy)
            }
            0xA => {
                // LD instruction (index)
                LoadAddress(addr)
            }
            0xB => {
                // JP instruction with offset
                JumpWithOffset(addr)
            }
            0xC => {
                // RND instruction
                RandAndImmediate(regx, imm)
            }
            0xD => {
                // DRW instruction
                Draw(regx, regy, nibble)
            }
            0xE => {
                // Input instructions (SKP and SKNP)
                match imm {
                    0x9E => SkipIfKeyPressed(regx),
                    0xA1 => SkipIfKeyNotPressed(regx),
                    _ => Unknown,
                }
            }
            0xF => {
                // Special loads and adds
                match imm {
                    0x07 => ReadDelayTimer(regx),
                    0x0A => StoreKeypress(regx),
                    0x15 => WriteDelayTimer(regx),
                    0x18 => WriteSoundTimer(regx),
                    0x1E => AddIndex(regx),
                    0x29 => SetSpriteLoc(regx),
                    0x33 => StoreBCD(regx),
                    0x55 => StoreRegisters(regx),
                    0x65 => ReadRegisters(regx),
                    _ => Unknown,
                }
            }
            _ => Unknown,
        }
    }
}
