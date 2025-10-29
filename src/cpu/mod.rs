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
    pc: u16, // Program Counter
    sp: u16, // Stack Counter

}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
        }
    }

    // Add all methods here

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    const FLAG_Z: u8 = 0b1000_0000;
    const FLAG_N: u8 = 0b0100_0000;
    const FLAG_H: u8 = 0b0010_0000;
    const FLAG_C: u8 = 0b0001_0000;

    pub fn get_flag_z(&self) -> bool {
        (self.f & Self::FLAG_Z) != 0
    }

    pub fn get_flag_n(&self) -> bool {
        (self.f & Self::FLAG_N) != 0
    }

    pub fn get_flag_h(&self) -> bool {
        (self.f & Self::FLAG_H) != 0
    }

    pub fn get_flag_c(&self) -> bool {
        (self.f & Self::FLAG_C) != 0
    }

    pub fn set_flag_z(&mut self, value: bool) {
        if value {
            // Set the bit to 1
            self.f |= Self::FLAG_Z;
        } else {
            // Set the bit to 0
            self.f &= !Self::FLAG_Z;
        }
    }

    pub fn set_flag_n(&mut self, value: bool) {
        if value {
            self.f |= Self::FLAG_N;
        } else {
            self.f &= !Self::FLAG_N;
        }
    }

    pub fn set_flag_h(&mut self, value: bool) {
        if value {
            self.f |= Self::FLAG_H;
        } else {
            self.f &= !Self::FLAG_H;
        }
    }

    pub fn set_flag_c(&mut self, value: bool) {
        if value{
            self.c |= Self::FLAG_C;
        } else {
            self.c &= !Self::FLAG_C;
        }
    }
}
