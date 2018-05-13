#[macro_use]
extern crate log;

mod cpu;
mod gameboy;
mod instruction;
mod mmu;
mod registers;
pub use gameboy::Gameboy;
