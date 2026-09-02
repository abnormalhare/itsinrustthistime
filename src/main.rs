#![feature(f128)]

use crate::{cpu::CPU, ram::RAM};

mod reg;
mod cpu;
mod ram;

fn main() {
    let mut ram = RAM::new("test.bin");
    let mut cpu = CPU::new(&mut ram);

    cpu.run(&mut ram);

    ram.deinit();
}
