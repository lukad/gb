use std::fmt::{self, Debug, Formatter};

macro_rules! instructions {
    ($($variant:ident($format:expr $(,$arg:ident: $arg_type:ty)*);)*) => {
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

pub enum Source {
    Register(Register),
    Address(Register),
}

impl Debug for Source {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Source::Register(register) => write!(f, "{:?}", register),
            Source::Address(register) => write!(f, "({:?})", register),
        }
    }
}

#[derive(Debug)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    HL,
}

instructions! {
    Nop("NOP");
    Add("ADD A, {:?}", operand: Source);
    // Nop("NOP");
    // LoadImmediateByte("LD {:?}, {:#04X?}", reg: ByteRegister, n: u8);
    // Load("LD {:?}, {:?}", r1: ByteRegister, r2: ByteRegister);
    // LoadImmediateWord("LD {:?}, {:#06X?}", r1: ByteRegister, nn: u16);
}
