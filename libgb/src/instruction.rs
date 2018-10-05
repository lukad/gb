use std::fmt::{self, Debug, Formatter};

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
    IncR("INC {:?}", target: Register);
    IncRR("INC {:?}", target: DoubleRegister);
    LdRN("LD {:?}, d8", target: Register);
}

use self::DoubleRegister::*;
use self::Instruction::*;
use self::Register::*;

impl Instruction {
    /// Decodes the given byte and returns `Ok(Instruction)`.
    /// If the byte cannot be decoded it is returned as `Err(byte)`.
    pub fn decode(byte: u8) -> Result<Self, u8> {
        match byte {
            0x00 => Ok(Nop()),
            0xC9 => Ok(Ret()),
            0x37 => Ok(Scf()),

            0x3C => Ok(IncR(A)),
            0x2C => Ok(IncR(L)),
            0x1C => Ok(IncR(E)),
            0x0C => Ok(IncR(C)),
            0x04 => Ok(IncR(B)),
            0x14 => Ok(IncR(D)),
            0x24 => Ok(IncR(H)),

            0x03 => Ok(IncRR(BC)),
            0x13 => Ok(IncRR(DE)),
            0x23 => Ok(IncRR(HL)),
            0xC3 => Ok(IncRR(SP)),

            0x3E => Ok(LdRN(A)),
            0x06 => Ok(LdRN(B)),
            0x0E => Ok(LdRN(C)),
            0x16 => Ok(LdRN(D)),
            0x1E => Ok(LdRN(E)),
            0x26 => Ok(LdRN(H)),
            0x2E => Ok(LdRN(L)),

            _ => Err(byte),
        }
    }
}
