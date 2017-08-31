extern crate rand;

mod cpu;
mod display;
mod memory;
mod keyboard;

use std::fs::File;
use std::{thread, time};
use std::io;
use std::io::prelude::*;

pub fn run(file: &mut File) {
    let memory = memory::BlockMemory::new(file);
    let mut cpu = cpu::Cpu::<memory::BlockMemory>::new(memory);
    loop {
        for _ in 0..10 {
            //println!("{:?}", cpu);
            //let _ = io::stdin().read(&mut [0u8]).unwrap();
            cpu.cycle();
        }
        thread::sleep(time::Duration::from_millis(17));
        cpu.decrement_timers();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
