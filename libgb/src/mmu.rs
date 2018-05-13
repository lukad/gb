use std::io::Read;

pub struct Mmu {
    mem: [u8; 0xFFFF],
}

impl Mmu {
    pub fn new() -> Mmu {
        debug!("Initializing MMU");

        Mmu { mem: [0; 0xFFFF] }
    }

    pub fn read<R: Read>(&mut self, mut reader: R, offset: u16) {
        let n = reader.read(&mut self.mem[(offset as usize)..]).unwrap();
        debug!("Read {} bytes at {:#06X?}", n, offset);
    }
}
