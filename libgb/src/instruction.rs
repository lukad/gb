use std::fmt::{self, Debug, Formatter};

use mmu::Mmu;

macro_rules! instructions {
    ($($variant:ident($format:expr $(,$arg:ident: $arg_type:ty)*);)*) => {
        #[derive(Clone, PartialEq)]
        pub enum Instruction {
            $($variant($($arg_type),*)),*
        }

        impl Debug for Instruction {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                match self {
                    $(
                        Instruction::$variant($($arg),*) => write!(f, $format, $($arg),*)
                    ),*
                }
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DoubleRegister {
    BC,
    DE,
    HL,
    SP,
}

instructions! {
    Nop("NOP");
    Ret("RET");
    Scf("SCF");
    Di("DI");
    IncR("INC {:?}", target: Register);
    IncRR("INC {:?}", target: DoubleRegister);
    LdRN("LD {:?}, {:02X}", target: Register, value: u8);
    Jmp("JP {:04X}", address: u16);
}

use self::DoubleRegister::*;
use self::Instruction::*;
use self::Register::*;

impl Instruction {
    /// Decodes the given byte and returns `Ok(Instruction)`.
    /// If the byte cannot be decoded it is returned as `Err(byte)`.
    pub fn decode(byte: u8, mmu: &Mmu, pc: u16) -> Result<Self, u8> {
        let next_addr = pc.wrapping_add(1);
        match byte {
            0x00 => Ok(Nop()),
            0xC9 => Ok(Ret()),
            0x37 => Ok(Scf()),
            0xF3 => Ok(Di()),

            0x3C => Ok(IncR(A)),
            0x04 => Ok(IncR(B)),
            0x0C => Ok(IncR(C)),
            0x14 => Ok(IncR(D)),
            0x1C => Ok(IncR(E)),
            0x24 => Ok(IncR(H)),
            0x2C => Ok(IncR(L)),

            0x03 => Ok(IncRR(BC)),
            0x13 => Ok(IncRR(DE)),
            0x23 => Ok(IncRR(HL)),
            0x33 => Ok(IncRR(SP)),

            0x3E => Ok(LdRN(A, mmu.read_byte(next_addr))),
            0x06 => Ok(LdRN(B, mmu.read_byte(next_addr))),
            0x0E => Ok(LdRN(C, mmu.read_byte(next_addr))),
            0x16 => Ok(LdRN(D, mmu.read_byte(next_addr))),
            0x1E => Ok(LdRN(E, mmu.read_byte(next_addr))),
            0x26 => Ok(LdRN(H, mmu.read_byte(next_addr))),
            0x2E => Ok(LdRN(L, mmu.read_byte(next_addr))),

            0xC3 => Ok(Jmp(mmu.read_word(next_addr).to_be())),

            _ => Err(byte),
        }
    }
}
