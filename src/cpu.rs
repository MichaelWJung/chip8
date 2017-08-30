use memory::Memory;

type Opcode = u16;

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

pub struct Cpu<M: Memory> {
    registers: Registers,
    memory: M,
}

impl<M: Memory> Cpu<M> {
    pub fn new(memory: M) -> Cpu<M> {
        Cpu {
            registers: Registers::new(),
            memory: memory,
        }
    }

    pub fn cycle(&mut self) {
        let opcode = self.fetch_opcode();
    }

    fn fetch_opcode(&self) -> Opcode {
        let pc = self.registers.pc;
        let byte1 = self.memory.read_byte(pc);
        let byte2 = self.memory.read_byte(pc + 1);
        (byte1 as u16) << 8 | (byte2 as u16)
    }

    fn execute_opcode(&mut self, opcode: Opcode) {
        match opcode {
            0x00e0 => self.cls(),
            0x00ee => self.ret(),
            0x1000...0x1FFF => self.jp(opcode),
            0x2000...0x2FFF => self.call(opcode),
            0x3000...0x3FFF => self.se_rc(opcode),
            0x4000...0x4FFF => self.sne_rc(opcode),
            0x5000...0x5FFF if opcode % 0xF == 0x0 => self.se_rr(opcode),
            0x6000...0x6FFF => self.ld_rc(opcode),
            0x7000...0x7FFF => self.add_rc(opcode),
            0x8000...0x8FFF if opcode % 0xF == 0x0 => self.ld_rr(opcode),
            0x8000...0x8FFF if opcode % 0xF == 0x1 => self.or(opcode),
            0x8000...0x8FFF if opcode % 0xF == 0x2 => self.and(opcode),
            0x8000...0x8FFF if opcode % 0xF == 0x3 => self.xor(opcode),
            0x8000...0x8FFF if opcode % 0xF == 0x4 => self.add_rr(opcode),
            0x8000...0x8FFF if opcode % 0xF == 0x5 => self.sub(opcode),
            0x8000...0x8FFF if opcode % 0xFF == 0x6 => self.shr(opcode),
            0x8000...0x8FFF if opcode % 0xF == 0x7 => self.subn(opcode),
            0x8000...0x8FFF if opcode % 0xFF == 0xE => self.shl(opcode),
            0x9000...0x9FFF if opcode % 0xF == 0x0 => self.sne_rr(opcode),
            0xA000...0xAFFF => self.ld_addr(opcode),
            0xB000...0xBFFF => self.jp2(opcode),
            0xC000...0xCFFF => self.rnd(opcode),
            0xD000...0xDFFF if opcode % 0xF != 0x0 => self.drw(opcode),
            0xE09E...0xEF9E if opcode % 0xFF == 0x9E => self.skp(opcode),
            0xE0A1...0xEFA1 if opcode % 0xFF == 0xA1 => self.sknp(opcode),
            0xF007...0xFF07 if opcode % 0xFF == 0x07 => self.ld_vx_dt(opcode),
            0xF00A...0xFF0A if opcode % 0xFF == 0x0A => self.ld_k(opcode),
            0xF015...0xFF15 if opcode % 0xFF == 0x15 => self.ld_dt_vx(opcode),
            0xF018...0xFF18 if opcode % 0xFF == 0x18 => self.ld_st_vx(opcode),
            0xF01E...0xFF1E if opcode % 0xFF == 0x1E => self.add_i_vx(opcode),
            0xF029...0xFF29 if opcode % 0xFF == 0x29 => self.ld_sprite(opcode),
            0xF033...0xFF33 if opcode % 0xFF == 0x33 => self.ld_bcd(opcode),
            0xF055...0xFF55 if opcode % 0xFF == 0x55 => self.ld_i_vx(opcode),
            0xF065...0xFF65 if opcode % 0xFF == 0x65 => self.ld_vx_i(opcode),
            _ => (),
        }
    }

    fn cls(&mut self) {
        // clear screen
    }

    fn ret(&mut self) {
        // return from subroutine call
    }

    fn jp(&mut self, opcode: Opcode) {
    }

    fn call(&mut self, opcode: Opcode) {
    }

    fn se_rc(&mut self, opcode: Opcode) {
    }

    fn sne_rc(&mut self, opcode: Opcode) {
    }

    fn se_rr(&mut self, opcode: Opcode) {
    }

    fn ld_rc(&mut self, opcode: Opcode) {
    }

    fn add_rc(&mut self, opcode: Opcode) {
    }

    fn ld_rr(&mut self, opcode: Opcode) {
    }

    fn or(&mut self, opcode: Opcode) {
    }

    fn and(&mut self, opcode: Opcode) {
    }

    fn xor(&mut self, opcode: Opcode) {
    }

    fn add_rr(&mut self, opcode: Opcode) {
    }

    fn sub(&mut self, opcode: Opcode) {
    }

    fn shr(&mut self, opcode: Opcode) {
    }

    fn subn(&mut self, opcode: Opcode) {
    }

    fn shl(&mut self, opcode: Opcode) {
    }

    fn sne_rr(&mut self, opcode: Opcode) {
    }

    fn ld_addr(&mut self, opcode: Opcode) {
    }

    fn jp2(&mut self, opcode: Opcode) {
    }

    fn rnd(&mut self, opcode: Opcode) {
    }

    fn drw(&mut self, opcode: Opcode) {
    }

    fn skp(&mut self, opcode: Opcode) {
    }

    fn sknp(&mut self, opcode: Opcode) {
    }

    fn ld_vx_dt(&mut self, opcode: Opcode) {
    }

    fn ld_k(&mut self, opcode: Opcode) {
    }

    fn ld_dt_vx(&mut self, opcode: Opcode) {
    }

    fn ld_st_vx(&mut self, opcode: Opcode) {
    }

    fn add_i_vx(&mut self, opcode: Opcode) {
    }

    fn ld_sprite(&mut self, opcode: Opcode) {
    }

    fn ld_bcd(&mut self, opcode: Opcode) {
    }

    fn ld_i_vx(&mut self, opcode: Opcode) {
    }

    fn ld_vx_i(&mut self, opcode: Opcode) {
    }
}
