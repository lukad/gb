use std::io::Read;

use mmu::Mmu;
use registers::Registers;

use instruction::{
    DoubleRegister::*,
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

    fn fetch_byte(&mut self) -> u8 {
        self.mmu.read_byte(self.v.pc)
    }

    pub fn step(&mut self) {
        let byte = self.fetch_byte();
        let ins = Instruction::decode(byte, &self.mmu, self.v.pc).unwrap_or_else(|byte| {
            debug!("{:?}", self.v);
            panic!("Could not decode byte: {:#04X}", byte);
        });
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
            Scf() => {
                self.v.set_subtract(false);
                self.v.set_half_carry(false);
                self.v.set_carry(true);
            }
            Di() => self.v.ime = false,
            Jmp(addr) => {
                self.v.pc = addr;
            }
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
            IncRR(BC) => {
                let r = self.v.bc().wrapping_add(1);
                self.v.set_bc(r);
            }
            IncRR(DE) => {
                let r = self.v.de().wrapping_add(1);
                self.v.set_de(r);
            }
            IncRR(HL) => {
                let r = self.v.hl().wrapping_add(1);
                self.v.set_hl(r);
            }
            IncRR(SP) => self.v.sp = self.v.sp.wrapping_add(1),
            LdRN(r, byte) => {
                self.v.pc = self.v.pc.wrapping_add(1);
                match r {
                    A => self.v.a = byte,
                    B => self.v.b = byte,
                    C => self.v.c = byte,
                    D => self.v.d = byte,
                    E => self.v.e = byte,
                    H => self.v.h = byte,
                    L => self.v.l = byte,
                }
            }
        }
    }
}
