pub struct CPU {
    // There are 8 Registers in the CPU
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    
    // 16-bit registers
    pc: u16, // Stack Pointer
    sp: ua6, // Program Counter

}

impl CPU {
    pub fn new() -> Self {
        CPU {}
    }
}
