use std::fs::File;
use std::io;

use cpu::Cpu;
use mmu::Mmu;
use registers::Registers;

pub struct Gameboy {
    cpu: Cpu,
}

impl Gameboy {
    pub fn new() -> Gameboy {
        debug!("Initializing Gameboy");

        let mmu = Mmu::new();
        let registers = Registers::new();
        let cpu = Cpu::new(mmu, registers);

        Gameboy { cpu: cpu }
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.step();
        }
    }

    /// Loads a ROM from disk into memory.
    pub fn load(&mut self, path: String) -> io::Result<()> {
        info!("Loading ROM {}", path);
        let f = File::open(path)?;
        self.cpu.load(f, 0);
        Ok(())
    }
}
