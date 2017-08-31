use display::Display;
use memory::Memory;
use keyboard::Keyboard;
use rand;
use rand::Rng;
use std::num::Wrapping;

type Opcode = u16;

#[derive(Debug)]
struct Registers {
    v: [u8; 16],
    stack: [u16; 16],
    i: u16,
    pc: u16,
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
}

impl Registers {
    fn new() -> Registers {
        Registers {
            v: [0; 16],
            stack: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}

#[derive(Debug)]
pub struct Cpu<M: Memory> {
    registers: Registers,
    memory: M,
    display: Display,
    keyboard: Keyboard,
}

impl<M: Memory> Cpu<M> {
    pub fn new(memory: M) -> Cpu<M> {
        Cpu {
            registers: Registers::new(),
            memory: memory,
            display: Display::new(),
            keyboard: Keyboard::new(),
        }
    }

    pub fn cycle(&mut self) {
        let opcode = self.fetch_opcode();
        self.execute_opcode(opcode);
    }

    pub fn decrement_timers(&mut self) {
        if self.registers.delay_timer > 0 {
            self.registers.delay_timer -= 1;
        }
        if self.registers.sound_timer > 0 {
            self.registers.sound_timer -= 1;
        }
    }

    fn fetch_opcode(&self) -> Opcode {
        let pc = self.registers.pc;
        let byte1 = self.memory.read_byte(pc);
        let byte2 = self.memory.read_byte(pc + 1);
        (byte1 as u16) << 8 | (byte2 as u16)
    }

    fn execute_opcode(&mut self, opcode: Opcode) {
        //println!("Executing opcode: {}", opcode);
        match opcode {
            0x00e0 => self.cls(),
            0x00ee => self.ret(),
            0x1000...0x1FFF => self.jp(opcode),
            0x2000...0x2FFF => self.call(opcode),
            0x3000...0x3FFF => self.se_rc(opcode),
            0x4000...0x4FFF => self.sne_rc(opcode),
            0x5000...0x5FFF if opcode & 0xF == 0x0 => self.se_rr(opcode),
            0x6000...0x6FFF => self.ld_rc(opcode),
            0x7000...0x7FFF => self.add_rc(opcode),
            0x8000...0x8FFF if opcode & 0xF == 0x0 => self.ld_rr(opcode),
            0x8000...0x8FFF if opcode & 0xF == 0x1 => self.or(opcode),
            0x8000...0x8FFF if opcode & 0xF == 0x2 => self.and(opcode),
            0x8000...0x8FFF if opcode & 0xF == 0x3 => self.xor(opcode),
            0x8000...0x8FFF if opcode & 0xF == 0x4 => self.add_rr(opcode),
            0x8000...0x8FFF if opcode & 0xF == 0x5 => self.sub(opcode),
            0x8000...0x8FFF if opcode & 0xFF == 0x6 => self.shr(opcode),
            0x8000...0x8FFF if opcode & 0xF == 0x7 => self.subn(opcode),
            0x8000...0x8FFF if opcode & 0xFF == 0xE => self.shl(opcode),
            0x9000...0x9FFF if opcode & 0xF == 0x0 => self.sne_rr(opcode),
            0xA000...0xAFFF => self.ld_addr(opcode),
            0xB000...0xBFFF => self.jp2(opcode),
            0xC000...0xCFFF => self.rnd(opcode),
            0xD000...0xDFFF if opcode & 0xF != 0x0 => self.drw(opcode),
            0xE09E...0xEF9E if opcode & 0xFF == 0x9E => self.skp(opcode),
            0xE0A1...0xEFA1 if opcode & 0xFF == 0xA1 => self.sknp(opcode),
            0xF007...0xFF07 if opcode & 0xFF == 0x07 => self.ld_vx_dt(opcode),
            0xF00A...0xFF0A if opcode & 0xFF == 0x0A => self.ld_k(opcode),
            0xF015...0xFF15 if opcode & 0xFF == 0x15 => self.ld_dt_vx(opcode),
            0xF018...0xFF18 if opcode & 0xFF == 0x18 => self.ld_st_vx(opcode),
            0xF01E...0xFF1E if opcode & 0xFF == 0x1E => self.add_i_vx(opcode),
            0xF029...0xFF29 if opcode & 0xFF == 0x29 => self.ld_sprite(opcode),
            0xF033...0xFF33 if opcode & 0xFF == 0x33 => self.ld_bcd(opcode),
            0xF055...0xFF55 if opcode & 0xFF == 0x55 => self.ld_i_vx(opcode),
            0xF065...0xFF65 if opcode & 0xFF == 0x65 => self.ld_vx_i(opcode),
            x => panic!("Opcode unknown: {:X}", x),
        }
    }

    fn cls(&mut self) {
        // clear screen
        self.display.clear();
        self.registers.pc += 2;
    }

    fn ret(&mut self) {
        // Return from a subroutine
        self.registers.sp -= 1;
        self.registers.pc = self.registers.stack[self.registers.sp as usize];
        self.registers.pc += 2;
    }

    fn jp(&mut self, opcode: Opcode) {
        // Jump to location at nnn
        self.registers.pc = Self::get_address(opcode);
    }

    fn jp2(&mut self, opcode: Opcode) {
        // Jump to location nnn + V0
        let v0 = self.registers.v[0] as u16;
        self.registers.pc = v0 + Self::get_address(opcode);
    }

    fn call(&mut self, opcode: Opcode) {
        // Call subroutine at nnn
        self.registers.stack[self.registers.sp as usize] = self.registers.pc;
        self.registers.sp += 1;
        self.registers.pc = Self::get_address(opcode);
    }

    fn se_rc(&mut self, opcode: Opcode) {
        // Skip next instruction if Vx == kk
        let reg = self.registers.v[Self::get_index_from_nibble(opcode, 3)];
        let byte = Self::get_low_byte(opcode);
        if reg == byte {
            self.registers.pc += 2;
        }
        self.registers.pc += 2;
    }

    fn sne_rc(&mut self, opcode: Opcode) {
        // Skip next instruction if Vx != kk
        let reg = self.registers.v[Self::get_index_from_nibble(opcode, 3)];
        let byte = Self::get_low_byte(opcode);
        if reg != byte {
            self.registers.pc += 2;
        }
        self.registers.pc += 2;
    }

    fn sne_rr(&mut self, opcode: Opcode) {
        // Skip next instruction if Vx != Vy
        let i1 = Self::get_index_from_nibble(opcode, 3);
        let i2 = Self::get_index_from_nibble(opcode, 2);
        if self.registers.v[i1] != self.registers.v[i2] {
            self.registers.pc += 2;
        }
        self.registers.pc += 2;
    }

    fn se_rr(&mut self, opcode: Opcode) {
        // Skip next instruction if Vx == Vy
        let reg1 = self.registers.v[Self::get_index_from_nibble(opcode, 3)];
        let reg2 = self.registers.v[Self::get_index_from_nibble(opcode, 2)];
        if reg1 == reg2 {
            self.registers.pc += 2;
        }
        self.registers.pc += 2;
    }

    fn ld_rc(&mut self, opcode: Opcode) {
        // Set Vx == kk
        let byte = Self::get_low_byte(opcode);
        self.registers.v[Self::get_index_from_nibble(opcode, 3)] = byte;
        self.registers.pc += 2;
    }

    fn ld_rr(&mut self, opcode: Opcode) {
        // Set Vx = Vy
        let reg2 = self.registers.v[Self::get_index_from_nibble(opcode, 2)];
        let reg1 = &mut self.registers.v[Self::get_index_from_nibble(opcode, 3)];
        *reg1 = reg2;
        self.registers.pc += 2;
    }

    fn ld_addr(&mut self, opcode: Opcode) {
        // Set I = nnn
        self.registers.i = Self::get_address(opcode);
        self.registers.pc += 2;
    }

    fn ld_vx_dt(&mut self, opcode: Opcode) {
        // Set Vx = delay timer value
        let i = Self::get_index_from_nibble(opcode, 3);
        self.registers.v[i] = self.registers.delay_timer;
        self.registers.pc += 2;
    }

    fn ld_k(&mut self, opcode: Opcode) {
        // Wait for a key press, store the value of the key in Vx.
        if let Some(key) = self.keyboard.any_key_pressed() {
            let i = Self::get_index_from_nibble(opcode, 3);
            self.registers.v[i] = key;
            self.registers.pc += 2;
        }
    }

    fn ld_dt_vx(&mut self, opcode: Opcode) {
        // Set delay timer = Vx
        let i = Self::get_index_from_nibble(opcode, 3);
        self.registers.delay_timer = self.registers.v[i];
        self.registers.pc += 2;
    }

    fn ld_st_vx(&mut self, opcode: Opcode) {
        // Set sound timer = Vx
        let i = Self::get_index_from_nibble(opcode, 3);
        self.registers.sound_timer = self.registers.v[i];
        self.registers.pc += 2;
    }

    fn ld_sprite(&mut self, opcode: Opcode) {
        // Set I = location of sprite for digit Vx
        let i = Self::get_index_from_nibble(opcode, 3);
        let val = self.registers.v[i] % 0x10;
        // The digit sprites are stored from memory location 0x0 onwards and are
        // 5 bytes long each
        self.registers.i = val as u16 * 0x500;
        self.registers.pc += 2;
    }

    fn ld_bcd(&mut self, opcode: Opcode) {
        // Store BCD representation of Vx in memory locations I, I+1, and I+2
        let i = Self::get_index_from_nibble(opcode, 3);
        let val = self.registers.v[i];
        self.memory.write_byte(self.registers.i, val / 100);
        self.memory.write_byte(self.registers.i + 1, val % 100 / 10);
        self.memory.write_byte(self.registers.i + 2, val % 10);
        self.registers.pc += 2;
    }

    fn ld_i_vx(&mut self, opcode: Opcode) {
        // Store registers V0 through Vx in memory starting at location I
        let x = Self::get_index_from_nibble(opcode, 3);
        for (i, val) in (&self.registers.v[..(x+1)]).iter().enumerate() {
            self.memory.write_byte(self.registers.i + i as u16, *val);
        }
        self.registers.pc += 2;
    }

    fn ld_vx_i(&mut self, opcode: Opcode) {
        // Read registers V0 through Vx from memory starting at location I
        let x = Self::get_index_from_nibble(opcode, 3);
        for (i, reg) in (&mut self.registers.v[..(x+1)]).iter_mut().enumerate() {
            *reg = self.memory.read_byte(self.registers.i + i as u16);
        }
        self.registers.pc += 2;
    }

    fn add_rc(&mut self, opcode: Opcode) {
        // Set Vx = Vx + kk
        let reg_index = Self::get_index_from_nibble(opcode, 3);
        let reg = &mut self.registers.v[reg_index];
        let val1 = Wrapping(*reg);
        let val2 = Wrapping(Self::get_low_byte(opcode));
        *reg = (val1 + val2).0;
        self.registers.pc += 2;
    }

    fn add_rr(&mut self, opcode: Opcode) {
        // Set Vx = Vx + Vy, set VF = carry
        let i1 = Self::get_index_from_nibble(opcode, 3);
        let i2 = Self::get_index_from_nibble(opcode, 2);
        let val1 = Wrapping(self.registers.v[i1]);
        let val2 = Wrapping(self.registers.v[i2]);
        let sum = val1 + val2;
        let carry = sum < val1 && sum < val2;
        self.registers.v[0xF] = carry as u8;
        self.registers.v[i1] = sum.0;
        self.registers.pc += 2;
    }

    fn add_i_vx(&mut self, opcode: Opcode) {
        // Set I = I + Vx
        let i = Self::get_index_from_nibble(opcode, 3);
        let vx = Wrapping(self.registers.v[i] as u16);
        let old = Wrapping(self.registers.i);
        self.registers.i = (old + vx).0;
        self.registers.pc += 2;
    }

    fn sub(&mut self, opcode: Opcode) {
        // Set Vx = Vx - Vy, set VF = NOT borrow
        let i1 = Self::get_index_from_nibble(opcode, 3);
        let i2 = Self::get_index_from_nibble(opcode, 2);
        let val1 = Wrapping(self.registers.v[i1]);
        let val2 = Wrapping(self.registers.v[i2]);
        let difference = val1 - val2;
        let borrow = val1 < val2; // or is it <= ??
        self.registers.v[0xF] = !borrow as u8;
        self.registers.v[i1] = difference.0;
        self.registers.pc += 2;
    }

    fn subn(&mut self, opcode: Opcode) {
        // Set Vx = Vy - Vx, set VF = NOT borrow
        let i1 = Self::get_index_from_nibble(opcode, 3);
        let i2 = Self::get_index_from_nibble(opcode, 2);
        let val1 = Wrapping(self.registers.v[i1]);
        let val2 = Wrapping(self.registers.v[i2]);
        let difference = val2 - val1;
        let borrow = val2 < val1; // or is it <= ??
        self.registers.v[0xF] = !borrow as u8;
        self.registers.v[i1] = difference.0;
        self.registers.pc += 2;
    }

    fn or(&mut self, opcode: Opcode) {
        // Set Vx = Vx OR Vy
        let reg2 = self.registers.v[Self::get_index_from_nibble(opcode, 2)];
        let reg1 = &mut self.registers.v[Self::get_index_from_nibble(opcode, 3)];
        *reg1 = *reg1 | reg2;
        self.registers.pc += 2;
    }

    fn and(&mut self, opcode: Opcode) {
        // Set Vx = Vx AND Vy
        let reg2 = self.registers.v[Self::get_index_from_nibble(opcode, 2)];
        let reg1 = &mut self.registers.v[Self::get_index_from_nibble(opcode, 3)];
        *reg1 = *reg1 & reg2;
        self.registers.pc += 2;
    }

    fn xor(&mut self, opcode: Opcode) {
        // Set Vx = Vx XOR Vy
        let reg2 = self.registers.v[Self::get_index_from_nibble(opcode, 2)];
        let reg1 = &mut self.registers.v[Self::get_index_from_nibble(opcode, 3)];
        *reg1 = *reg1 ^ reg2;
        self.registers.pc += 2;
    }

    fn shr(&mut self, opcode: Opcode) {
        // Set Vx = Vx SHR 1
        let i = Self::get_index_from_nibble(opcode, 3);
        let val = self.registers.v[i];
        self.registers.v[0xF] = val % 2;
        self.registers.v[i] = val >> 1;
        self.registers.pc += 2;
    }

    fn shl(&mut self, opcode: Opcode) {
        // Set Vx = Vx SHR 1
        let i = Self::get_index_from_nibble(opcode, 3);
        let val = self.registers.v[i];
        let msb = (val & 0b1000_0000) > 0;
        self.registers.v[0xF] = msb as u8;
        self.registers.v[i] = val << 1;
        self.registers.pc += 2;
    }

    fn rnd(&mut self, opcode: Opcode) {
        // Set Vx = random byte AND kk
        let mut rng = rand::thread_rng();
        let rand_byte = rng.gen::<u8>();
        let result = rand_byte & Self::get_low_byte(opcode);
        let i = Self::get_index_from_nibble(opcode, 3);
        self.registers.v[i] = result;
        self.registers.pc += 2;
    }

    fn drw(&mut self, opcode: Opcode) {
        let x = Self::get_index_from_nibble(opcode, 3);
        let y = Self::get_index_from_nibble(opcode, 2);
        let lines = Self::get_index_from_nibble(opcode, 1);
        let sprite = self.memory.read_block(self.registers.i, lines);
        let erased_pixel = self.display.draw_sprite(x, y, sprite);
        self.registers.v[0xF] = erased_pixel as u8;
        self.registers.pc += 2;
    }

    fn skp(&mut self, opcode: Opcode) {
        let i = Self::get_index_from_nibble(opcode, 3);
        let key = self.registers.v[i];
        if self.keyboard.is_pressed(key) {
            self.registers.pc += 2;
        }
        self.registers.pc += 2;
    }

    fn sknp(&mut self, opcode: Opcode) {
        let i = Self::get_index_from_nibble(opcode, 3);
        let key = self.registers.v[i];
        if !self.keyboard.is_pressed(key) {
            self.registers.pc += 2;
        }
        self.registers.pc += 2;
    }

    fn get_address(opcode: Opcode) -> u16 {
        opcode & 0xFFF
    }

    fn get_index_from_nibble(opcode: Opcode, nibble: u8) -> usize {
        let shift = (nibble - 1) * 4;
        ((opcode & (0xF << shift)) >> shift) as usize
    }

    fn get_low_byte(opcode: Opcode) -> u8 {
        (opcode & 0xFF) as u8
    }
}
