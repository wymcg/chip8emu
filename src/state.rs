use crate::input::Input;
use crate::instructions::Instruction::*;
use crate::instructions::{Address, Immediate, Instruction, Register};
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{BufReader, Read};

const MEM_SIZE: usize = 4096;
const STACK_SIZE: usize = 1024;
const PROGMEM_START: u16 = 0x200;
const FONTMEM_START: u16 = 0x000;

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
    pub memory: Memory,
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
                input: 0,
            },
            memory: Memory {
                ram: [0; MEM_SIZE],
                stack: [0; STACK_SIZE],
                vram: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            },
        }
    }

    /// Load a rom into memory
    pub fn load_rom(mut self, path: &str) -> Self {
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

    pub fn load_font(mut self, path: &str) -> Self {
        // open the file
        let file: File = File::open(path).expect("Unable to open font file!");

        // make the vec to hold the bytes
        let mut bytes: Vec<u8> = Vec::new();

        // read the file into the bytes vector
        BufReader::new(file)
            .read_to_end(&mut bytes)
            .expect("Unable to read file!");

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
    pub fn get_display(&self) -> &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT] {
        &self.memory.vram
    }

    /// Update the inputs
    pub fn change_input(&mut self, input: Input) {
        // TODO implement
    }

    /// Do the next instruction and return the result, containing the opcode that was just dealt with
    pub fn do_next_instruction(&mut self) -> Result<u16, u16> {
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
                self.registers.pc = self.memory.stack[self.registers.sp as usize];
            }
            Jump(addr) => {
                // jump to the given address
                // set the program counter to be the given address
                self.registers.pc = addr - 0x2;
            }
            JumpWithOffset(addr) => {
                // jump to the given address, offset by the value in V0
                self.registers.pc = addr + self.registers.v[0x0] as u16;
            }
            Call(addr) => {
                // call subroutine at the given address
                // put the current PC at the top of the stack
                self.memory.stack[self.registers.sp as usize] = self.registers.pc;
                self.registers.sp += 1;

                // replace the current PC with the given address
                self.registers.pc = addr - 0x2;
            }
            SkipEqualImm(reg, imm) => {
                // if the contents of the given register is equal to the immediate,
                // point the PC past the next instruction
                if self.registers.v[reg as usize] == imm {
                    self.registers.pc += 2;
                }
            }
            SkipNotEqualImm(reg, imm) => {
                // if the contents of the given register is not equal to the immediate,
                // point the PC past the next instruction
                if self.registers.v[reg as usize] != imm {
                    self.registers.pc += 2;
                }
            }
            SkipEqualReg(regx, regy) => {
                // if the contents of the two registers are the same,
                // point the PC past the next instruction
                if self.registers.v[regx as usize] == self.registers.v[regy as usize] {
                    self.registers.pc += 2;
                }
            }
            SkipNotEqualReg(regx, regy) => {
                // if th contents of the two registers are not the same,
                // point the PC past the next instruction
                if self.registers.v[regx as usize] != self.registers.v[regy as usize] {
                    self.registers.pc += 2;
                }
            }
            LoadImm(reg, imm) => {
                // load an immediate value into a register
                self.registers.v[reg as usize] = imm;
            }
            LoadReg(regx, regy) => {
                // load the contents of one register into another
                self.registers.v[regx as usize] = self.registers.v[regy as usize];
            }
            LoadAddress(addr) => {
                // load the index register with the given address
                self.registers.i = addr;
            }
            ReadDelayTimer(reg) => {
                // read the delay timer into a register
                self.registers.v[reg as usize] = self.registers.dt;
            }
            WriteDelayTimer(reg) => {
                // write the delay timer with the contents of a register
                self.registers.dt = self.registers.v[reg as usize];
            }
            WriteSoundTimer(reg) => {
                // write the sound timer with the contents of a register
                self.registers.st = self.registers.v[reg as usize];
            }
            AddImm(reg, imm) => {
                // get the result
                self.registers.v[reg as usize] = self.registers.v[reg as usize].wrapping_add(imm);
            }
            AddReg(regx, regy) => {
                // add together VX and VY and put the result in VX
                // get the result
                let (result, overflow): (u8, bool) = self.registers.v[regx as usize]
                    .overflowing_add(self.registers.v[regy as usize]);

                // check if was an overflow and set the VF register if so
                if overflow {
                    self.registers.v[0xF] = 0x1
                } else {
                    self.registers.v[0xF] = 0x0;
                }

                // set VX with the result
                self.registers.v[regx as usize] = result;
            }
            AddIndex(reg) => {
                // add I to VX and store in I
                self.registers.i = self
                    .registers
                    .i
                    .wrapping_add(self.registers.v[reg as usize] as u16);
            }
            SubReg(regx, regy) => {
                // subtract VY from VX and put the result in VX
                // get the result
                let (result, borrow): (u8, bool) = self.registers.v[regx as usize]
                    .overflowing_sub(self.registers.v[regy as usize]);

                // set VF depending on whether not there was a borrow
                if borrow {
                    self.registers.v[0xF] = 0x0;
                } else {
                    self.registers.v[0xF] = 0x1;
                }

                // store the result in VX
                self.registers.v[regx as usize] = result;
            }
            SubNReg(regx, regy) => {
                // subtract VX from VY and put the result in VY
                // get the result
                let (result, borrow): (u8, bool) = self.registers.v[regy as usize]
                    .overflowing_sub(self.registers.v[regx as usize]);

                // set VF depending on whether or not there was a borrow
                if borrow {
                    self.registers.v[0xF] = 0x0;
                } else {
                    self.registers.v[0xF] = 0x1;
                }

                // store the result in VX
                self.registers.v[regx as usize] = result;
            }
            ShiftRightReg(reg) => {
                // logical shift right by one
                // set VF with the lsb
                self.registers.v[0xF] = self.registers.v[reg as usize] & 0x01;

                // shift the register right 1
                self.registers.v[reg as usize] >>= 1;
            }
            ShiftLeftReg(reg) => {
                // logical shift left by one
                // set VF with the msb
                self.registers.v[0xF] = self.registers.v[reg as usize] & 0x80;

                // shift the register left one
                self.registers.v[reg as usize] <<= 1;
            }
            OrReg(regx, regy) => {
                // or together VX and VY and put the result in VX
                self.registers.v[regx as usize] |= self.registers.v[regy as usize];
            }
            AndReg(regx, regy) => {
                // and together VX and VY and put the result in VX
                self.registers.v[regx as usize] &= self.registers.v[regy as usize];
            }
            XorReg(regx, regy) => {
                // xor together VX and VY and put the result in VX
                self.registers.v[regx as usize] ^= self.registers.v[regy as usize];
            }
            RandAndImmediate(reg, imm) => {
                // generate a random value, and with imm, and store in VX
                self.registers.v[reg as usize] = thread_rng().gen::<u8>() & imm;
            }
            Draw(regx, regy, imm) => {
                // reset VF
                self.registers.v[0xF] = 0x0;

                // get x and y to start drawing the sprite
                let start_x: u8 = self.registers.v[regx as usize];
                let start_y: u8 = self.registers.v[regy as usize];

                for row in 0..imm {
                    for col in 0..8 {
                        // get the new pixel state
                        let pixel_state: bool = (self.memory.ram[self.registers.i as usize + row as usize]
                            & (0x1 << (7-col))) > 0;

                        // get the x and y for this pixel
                        let x = (start_x as usize + col as usize) % DISPLAY_WIDTH;
                        let y = (start_y as usize + row as usize) % DISPLAY_HEIGHT;

                        // check if the vram differs from the current pixel state (collision)
                        if self.memory.vram[y][x] != pixel_state {
                            // set the VF register
                            self.registers.v[0xF] = 0x1;

                            // write vram
                            self.memory.vram[y][x] = pixel_state;
                        }

                    }
                }
            }
            SetSpriteLoc(reg) => {
                // set I with the sprite info for the character in reg
                self.registers.i = self.registers.v[reg as usize] as u16 * 0x05;
                // each sprite is 5 bytes long
            }
            SkipIfKeyPressed(reg) => {
                // skip the next instruction if the input specified in the register is pressed
                if self.registers.input & (0x1 << self.registers.v[reg as usize]) > 0 {
                    self.registers.pc += 2;
                }
            }
            SkipIfKeyNotPressed(reg) => {
                // skip the next instruction if the input specified in the register is not pressed
                if self.registers.input & (0x1 << self.registers.v[reg as usize]) == 0 {
                    self.registers.pc += 2;
                }
            }
            StoreBCD(reg) => {
                // store BCD representation of VX in I, I+1, and I+2
                // get the hundreds, tens, and ones places
                let hundreds: u8 = reg / 100;
                let tens: u8 = (reg % 100) / 10;
                let ones: u8 = reg % 10;

                self.memory.ram[self.registers.i as usize] = hundreds;
                self.memory.ram[self.registers.i as usize + 1] = tens;
                self.memory.ram[self.registers.i as usize + 2] = ones;
            }
            StoreRegisters(reg) => {
                // store registers V0-VX in memory starting at I
                for r in 0..reg {
                    self.memory.ram[self.registers.i as usize + r as usize] =
                        self.registers.v[r as usize];
                }
            }
            ReadRegisters(reg) => {
                // populate registers V0-VX with data starting from I
                for r in 0..reg {
                    self.registers.v[r as usize] =
                        self.memory.ram[self.registers.i as usize + r as usize];
                }
            }
            _ => {
                return Err(current_opcode);
            }
        }

        // point the PC to the next instruction
        self.registers.pc += 2;

        Ok(current_opcode)
    }

    fn get_current_opcode(&self) -> u16 {
        ((self.memory.ram[self.registers.pc as usize] as u16) << 8)
            | (self.memory.ram[self.registers.pc as usize + 1] as u16)
    }

    fn get_current_instruction(&self) -> Instruction {
        // get the full opcode
        let opcode: u16 = self.get_current_opcode();

        // get the opcode components
        let inst_word: u8 = ((opcode & 0xF000) >> 12) as u8;
        let addr: Address = opcode & 0x0FFF;
        let nibble: u8 = (opcode & 0x000F) as u8;
        let imm: Immediate = (opcode & 0x00FF) as u8;
        let regx: Register = ((opcode & 0x0F00) >> 8) as u8;
        let regy: Register = ((opcode & 0x00F0) >> 4) as u8;

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
                    0x6 => ShiftRightReg(regx),
                    0x7 => SubNReg(regx, regy),
                    0xE => ShiftLeftReg(regx),
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
