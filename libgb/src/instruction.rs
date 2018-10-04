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
    IncR("INC {:?}", operand: Register);
}

impl Instruction {
    pub fn decode(byte: u8) -> Self {
        match byte {
            0x00 => Instruction::Nop(),
            0xC9 => Instruction::Ret(),

            0x3C => Instruction::IncR(Register::A),
            0x2C => Instruction::IncR(Register::L),
            0x1C => Instruction::IncR(Register::E),
            0x0C => Instruction::IncR(Register::C),
            0x04 => Instruction::IncR(Register::B),
            0x14 => Instruction::IncR(Register::D),
            0x24 => Instruction::IncR(Register::H),
            _ => panic!("Could not decode 0x{:X?}!", byte),
        }
    }
}
