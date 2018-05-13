use std::io::Read;

use mmu::Mmu;
use registers::Registers;

use instruction;

pub struct Cpu {
    mmu: Mmu,
    v: Registers,
}

impl Cpu {
    pub fn new(mmu: Mmu, registers: Registers) -> Cpu {
        debug!("Initializing CPU");

        let x =
            instruction::Instruction::Add(instruction::Source::Address(instruction::Register::HL));
        debug!("{:?}", x);

        Cpu {
            mmu: mmu,
            v: registers,
        }
    }

    pub fn read<R: Read>(&mut self, reader: R, offset: u16) {
        self.mmu.read(reader, offset);
    }
}
