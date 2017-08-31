use std::fs::File;
use std::io::Read;
use std::fmt;

pub trait Memory {
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, value: u8);
    fn read_block(&self, address: u16, size: usize) -> &[u8];
}

pub struct BlockMemory {
    memory: [u8; 4096],
}

impl BlockMemory {
    pub fn new(file: &mut File) -> BlockMemory {
        let mut memory = BlockMemory { memory: [0; 4096] };
        memory.load_rom(file);
        memory
    }

    fn load_rom(&mut self, file: &mut File) {
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        let size = bytes.len();
        for i in 0..size {
            self.memory[0x200 + i] = bytes[i];
        }
    }
}

impl Memory for BlockMemory {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    fn read_block(&self, address: u16, size: usize) -> &[u8] {
        let address = address as usize;
        &self.memory[address..(address + size)]
    }
}

impl fmt::Debug for BlockMemory {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        //self.memory[..].fmt(formatter)
        self.memory[0].fmt(formatter)
    }
}
