use std::io::Read;

use mmu::Mmu;
use registers::Registers;

use instruction::{
    Instruction::{self, *},
    Register::*,
};

pub struct Cpu {
    mmu: Mmu,
    v: Registers,
}

impl Cpu {
    pub fn new(mmu: Mmu, registers: Registers) -> Cpu {
        debug!("Initializing CPU");

        Cpu {
            mmu: mmu,
            v: registers,
        }
    }

    pub fn load<R: Read>(&mut self, reader: R, offset: u16) {
        self.mmu.load(reader, offset).unwrap();
    }

    pub fn step(&mut self) {
        let x = self.mmu.read_byte(self.v.pc);
        let ins = Instruction::decode(x);
        self.v.pc += 1;
        debug!("{:?}", ins);
        self.execute(ins);
    }

    fn pop(&mut self) -> u16 {
        let result = self.mmu.read_word(self.v.sp);
        self.v.sp.wrapping_add(2);
        result
    }

    fn execute(&mut self, ins: Instruction) {
        match ins.clone() {
            Nop() => (),
            Ret() => self.v.pc = self.pop(),
            IncR(register) => {
                let new = match register {
                    A => {
                        self.v.a = self.v.a.wrapping_add(1);
                        self.v.a
                    }
                    B => {
                        self.v.b = self.v.b.wrapping_add(1);
                        self.v.b
                    }
                    C => {
                        self.v.c = self.v.c.wrapping_add(1);
                        self.v.c
                    }
                    D => {
                        self.v.d = self.v.d.wrapping_add(1);
                        self.v.d
                    }
                    E => {
                        self.v.e = self.v.e.wrapping_add(1);
                        self.v.e
                    }
                    H => {
                        self.v.h = self.v.h.wrapping_add(1);
                        self.v.h
                    }
                    L => {
                        self.v.l = self.v.l.wrapping_add(1);
                        self.v.l
                    }
                };
                self.v.set_zero(new == 0);
                self.v.set_subtract(false);
                self.v.set_half_carry(new & 0x0F == 0);
            }
        }
    }
}
