use std::default::Default;

#[derive(Default, Debug)]
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
}

impl Registers {
    pub fn new() -> Registers {
        debug!("Initializing registers");
        Registers {
            pc: 0x0100,
            sp: 0xFFFE,
            ..Default::default()
        }
    }

    pub fn set(&mut self, bit: u8, b: bool) {
        self.flags |= (if b { 1 } else { 0 }) << bit;
    }

    pub fn set_zero(&mut self, b: bool) {
        self.set(7, b);
    }

    pub fn set_subtract(&mut self, b: bool) {
        self.set(6, b);
    }

    pub fn set_half_carry(&mut self, b: bool) {
        self.set(5, b);
    }
}
