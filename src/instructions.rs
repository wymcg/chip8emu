pub type Address = usize;
pub type Immediate = u8;
pub type Register = usize;

#[derive(Debug)]
pub enum Instruction {
    // emulator special
    Unknown,

    // system
    Sys(Address),
    Cls,
    Ret,
    Jump(Address),
    JumpWithOffset(Address),
    Call(Address),

    // branching
    SkipEqualImm(Register, Immediate),
    SkipEqualReg(Register, Register),
    SkipNotEqualImm(Register, Immediate),
    SkipNotEqualReg(Register, Register),

    // loads
    LoadImm(Register, Immediate),
    LoadReg(Register, Register),
    LoadAddress(Address),
    SetSpriteLoc(Register),
    StoreBCD(Register),
    StoreRegisters(Register),
    ReadRegisters(Register),

    // arithmetic
    AddImm(Register, Immediate),
    AddReg(Register, Register),
    AddIndex(Register),
    SubReg(Register, Register),
    SubNReg(Register, Register),

    // logic
    OrReg(Register, Register),
    AndReg(Register, Register),
    XorReg(Register, Register),
    ShiftRightReg(Register, Register),
    ShiftLeftReg(Register, Register),

    // special
    RandAndImmediate(Register, Immediate),
    Draw(Register, Register, Immediate),
    SkipIfKeyPressed(Register),
    SkipIfKeyNotPressed(Register),
    StoreKeypress(Register),

    // timers
    ReadDelayTimer(Register),
    WriteDelayTimer(Register),
    WriteSoundTimer(Register),
}
