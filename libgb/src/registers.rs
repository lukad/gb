use std::default::Default;
use std::fmt;

#[derive(Default)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub flags: u8,
    pub sp: u16,
    pub pc: u16,
    pub ime: bool,
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Registers {{ a: {:#04X}, b: {:#04X}, c: {:#04X}, d: {:#04X}, e: {:#04X}, h: {:#04X}, l: {:#04X}, flags: {:#010b}, sp: {:#06X}, pc: {:#06X} ime: {:?} }}", self.a, self.b, self.c, self.d, self.e, self.h, self.l, self.flags, self.sp, self.pc, self.ime)
    }
}

impl Registers {
    pub fn new() -> Registers {
        debug!("Initializing registers");
        Registers {
            pc: 0x0100,
            sp: 0xFFFE,
            ime: true,
            ..Default::default()
        }
    }

    pub fn bc(&mut self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }

    pub fn de(&mut self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }

    pub fn hl(&mut self) -> u16 {
        (self.h as u16) | (self.l as u16)
    }

    pub fn set_bc(&mut self, word: u16) {
        self.b = (word >> 8) as u8;
        self.c = (word & 0xFF) as u8;
    }

    pub fn set_de(&mut self, word: u16) {
        self.d = (word >> 8) as u8;
        self.e = (word & 0xFF) as u8;
    }

    pub fn set_hl(&mut self, word: u16) {
        self.h = (word >> 8) as u8;
        self.l = (word & 0xFF) as u8;
    }

    pub fn set_flag(&mut self, bit: u8, b: bool) {
        self.flags |= (if b { 1 } else { 0 }) << bit;
    }

    pub fn set_zero(&mut self, b: bool) {
        self.set_flag(7, b);
    }

    pub fn set_subtract(&mut self, b: bool) {
        self.set_flag(6, b);
    }

    pub fn set_half_carry(&mut self, b: bool) {
        self.set_flag(5, b);
    }

    pub fn set_carry(&mut self, b: bool) {
        self.set_flag(04, b);
    }
}
