use std::io;

pub struct Mmu {
    mem: [u8; 0xFFFF],
}

impl Mmu {
    pub fn new() -> Mmu {
        debug!("Initializing MMU");

        Mmu { mem: [0; 0xFFFF] }
    }

    pub fn load<R: io::Read>(&mut self, mut reader: R, offset: u16) -> io::Result<usize> {
        let n = reader.read(&mut self.mem[(offset as usize)..])?;
        debug!("Loaded {} bytes at {:#06X?}", n, offset);
        Ok(n)
    }

    pub fn read_byte(&self, index: u16) -> u8 {
        self.mem[index as usize]
    }

    pub fn read_word(&self, index: u16) -> u16 {
        (self.mem[index as usize] as u16) << 8 | (self.mem[(index as usize).wrapping_add(1)] as u16)
    }
}
