#![feature(f128)]

use crate::{cpu::CPU, ram::RAM};

mod reg;
mod cpu;
mod ram;

fn main() {
    let mut cpu = CPU::new();
    let mut ram = RAM::new("test.bin");

    cpu.run(&mut ram);

    ram.deinit();
}
