mod cpu;
mod memory;

use cpu::CPU;
use memory::Memory;

fn main() {
    println!("Game Boy Emulator Starting...");

    let cpu = CPU::new();
    let memory = Memory::new();

    println!("CPU and Memory initialized!");
}