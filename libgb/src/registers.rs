use std::default::Default;

#[derive(Default)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    flags: u8,
    sp: u16,
    pc: u16,
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
}
