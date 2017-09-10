use audio::AudioDevice;
use display::Display;
use memory::{BlockMemory, Memory};
use keyboard::Keyboard;
use rand;
use rand::Rng;
use std::num::Wrapping;

#[derive(Copy, Clone)]
struct Opcode {
    code: u16,
}

impl Opcode {
    fn new(code: u16) -> Opcode {
        Opcode { code }
    }

    fn get_address(&self) -> u16 {
        self.code & 0xFFF
    }

    fn get_index_from_nibble(&self, nibble: u8) -> usize {
        let shift = (nibble - 1) * 4;
        ((self.code & (0xF << shift)) >> shift) as usize
    }

    fn get_low_byte(&self) -> u8 {
        (self.code & 0xFF) as u8
    }
}

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

struct Components<'a, 'b: 'a, 'c: 'a> {
    registers: &'a mut Registers,
    memory: &'a mut BlockMemory,
    display: &'a mut Display<'c>,
    keyboard: &'a mut Keyboard<'b>,
    audio_device: &'a mut AudioDevice,
}

pub struct Cpu<'a, 'b> {
    registers: Registers,
    memory: BlockMemory,
    pub display: Display<'a>,
    keyboard: Keyboard<'b>,
    audio_device: AudioDevice,
}

impl<'a, 'b> Cpu<'a, 'b> {
    pub fn new(
        memory: BlockMemory,
        display: Display<'a>,
        keyboard: Keyboard<'b>,
        audio_device: AudioDevice,
    ) -> Cpu<'a, 'b> {
        Cpu {
            registers: Registers::new(),
            memory,
            display,
            keyboard,
            audio_device,
        }
    }

    pub fn cycle(&mut self) {
        self.keyboard.check_events();
        let opcode = self.fetch_opcode();
        self.execute_opcode(opcode);
    }

    pub fn decrement_timers(&mut self) {
        if self.registers.delay_timer > 0 {
            self.registers.delay_timer -= 1;
        }
        if self.registers.sound_timer > 0 {
            self.registers.sound_timer -= 1;
            if self.registers.sound_timer == 0 {
                self.audio_device.pause();
            }
        }
    }

    fn fetch_opcode(&self) -> Opcode {
        let pc = self.registers.pc;
        let byte1 = self.memory.read_byte(pc);
        let byte2 = self.memory.read_byte(pc + 1);
        Opcode::new((byte1 as u16) << 8 | (byte2 as u16))
    }

    fn execute_opcode(&mut self, opcode: Opcode) {
        match opcode.code {
            0x00e0 => self.create_and_execute::<Cls>(opcode),
            0x00ee => self.create_and_execute::<Ret>(opcode),
            0x1000...0x1FFF => self.create_and_execute::<Jp>(opcode),
            0x2000...0x2FFF => self.create_and_execute::<Call>(opcode),
            0x3000...0x3FFF => self.create_and_execute::<SeXkk>(opcode),
            0x4000...0x4FFF => self.create_and_execute::<SneXkk>(opcode),
            0x5000...0x5FFF if opcode.code & 0xF == 0x0 => self.create_and_execute::<SeXy>(opcode),
            0x6000...0x6FFF => self.create_and_execute::<LdXkk>(opcode),
            0x7000...0x7FFF => self.create_and_execute::<AddXkk>(opcode),
            0x8000...0x8FFF if opcode.code & 0xF == 0x0 => self.create_and_execute::<LdXy>(opcode),
            0x8000...0x8FFF if opcode.code & 0xF == 0x1 => self.create_and_execute::<Or>(opcode),
            0x8000...0x8FFF if opcode.code & 0xF == 0x2 => self.create_and_execute::<And>(opcode),
            0x8000...0x8FFF if opcode.code & 0xF == 0x3 => self.create_and_execute::<Xor>(opcode),
            0x8000...0x8FFF if opcode.code & 0xF == 0x4 => self.create_and_execute::<AddXy>(opcode),
            0x8000...0x8FFF if opcode.code & 0xF == 0x5 => self.create_and_execute::<Sub>(opcode),
            0x8000...0x8FFF if opcode.code & 0xFF == 0x6 => self.create_and_execute::<Shr>(opcode),
            0x8000...0x8FFF if opcode.code & 0xF == 0x7 => self.create_and_execute::<Subn>(opcode),
            0x8000...0x8FFF if opcode.code & 0xFF == 0xE => self.create_and_execute::<Shl>(opcode),
            0x9000...0x9FFF if opcode.code & 0xF == 0x0 => self.create_and_execute::<SneXy>(opcode),
            0xA000...0xAFFF => self.create_and_execute::<LdINnn>(opcode),
            0xB000...0xBFFF => self.create_and_execute::<Jp2>(opcode),
            0xC000...0xCFFF => self.create_and_execute::<Rnd>(opcode),
            0xD000...0xDFFF if opcode.code & 0xF != 0x0 => self.create_and_execute::<Drw>(opcode),
            0xE09E...0xEF9E if opcode.code & 0xFF == 0x9E => self.create_and_execute::<Skp>(opcode),
            0xE0A1...0xEFA1 if opcode.code & 0xFF == 0xA1 => self.create_and_execute::<Sknp>(opcode),
            0xF007...0xFF07 if opcode.code & 0xFF == 0x07 => self.create_and_execute::<LdXDt>(opcode),
            0xF00A...0xFF0A if opcode.code & 0xFF == 0x0A => self.create_and_execute::<LdKey>(opcode),
            0xF015...0xFF15 if opcode.code & 0xFF == 0x15 => self.create_and_execute::<LdDtX>(opcode),
            0xF018...0xFF18 if opcode.code & 0xFF == 0x18 => self.create_and_execute::<LdStX>(opcode),
            0xF01E...0xFF1E if opcode.code & 0xFF == 0x1E => self.create_and_execute::<AddIX>(opcode),
            0xF029...0xFF29 if opcode.code & 0xFF == 0x29 => self.create_and_execute::<LdXSprite>(opcode),
            0xF033...0xFF33 if opcode.code & 0xFF == 0x33 => self.create_and_execute::<LdBcd>(opcode),
            0xF055...0xFF55 if opcode.code & 0xFF == 0x55 => self.create_and_execute::<LdIX>(opcode),
            0xF065...0xFF65 if opcode.code & 0xFF == 0x65 => self.create_and_execute::<LdXI>(opcode),
            x => panic!("Opcode unknown: {:X}", x),
        }
    }

    fn create_and_execute<Op: OpConstruct + OpExecute>(&mut self, opcode: Opcode) {
        let op = Op::new(opcode);
        let components = Components {
            registers: &mut self.registers,
            memory: &mut self.memory,
            display: &mut self.display,
            keyboard: &mut self.keyboard,
            audio_device: &mut self.audio_device,
        };
        op.execute(components);
    }
}

trait OpConstruct {
    fn new(opcode: Opcode) -> Self;
}

trait OpExecute {
    fn execute(&self, c: Components);
}

macro_rules! create_opcode_struct {
    ($name:ident) => {
        struct $name {}

        impl OpConstruct for $name {
            fn new(_: Opcode) -> Self {
                $name {}
            }
        }
    }
}

macro_rules! create_opcode_struct_nnn {
    ($name:ident) => {
        struct $name {
            nnn: u16,
        }

        impl OpConstruct for $name {
            fn new(opcode: Opcode) -> Self {
                $name { nnn: opcode.get_address() }
            }
        }
    }
}

macro_rules! create_opcode_struct_x {
    ($name:ident) => {
        struct $name {
            x: usize,
        }

        impl OpConstruct for $name {
            fn new(opcode: Opcode) -> Self {
                $name { x: opcode.get_index_from_nibble(3) }
            }
        }
    }
}

macro_rules! create_opcode_struct_xy {
    ($name:ident) => {
        struct $name {
            x: usize,
            y: usize,
        }

        impl OpConstruct for $name {
            fn new(opcode: Opcode) -> Self {
                $name {
                    x: opcode.get_index_from_nibble(3),
                    y: opcode.get_index_from_nibble(2),
                }
            }
        }
    }
}

macro_rules! create_opcode_struct_xyn {
    ($name:ident) => {
        struct $name {
            x: usize,
            y: usize,
            n: usize,
        }

        impl OpConstruct for $name {
            fn new(opcode: Opcode) -> Self {
                $name {
                    x: opcode.get_index_from_nibble(3),
                    y: opcode.get_index_from_nibble(2),
                    n: opcode.get_index_from_nibble(1),
                }
            }
        }
    }
}

macro_rules! create_opcode_struct_xkk {
    ($name:ident) => {
        struct $name {
            x: usize,
            kk: u8,
        }

        impl OpConstruct for $name {
            fn new(opcode: Opcode) -> Self {
                $name {
                    x: opcode.get_index_from_nibble(3),
                    kk: opcode.get_low_byte(),
                }
            }
        }
    }
}

// Clear screen
create_opcode_struct!(Cls);
impl OpExecute for Cls {
    fn execute(&self, c: Components) {
        c.display.clear();
        c.registers.pc += 2;
    }
}

// Return from a subroutine
create_opcode_struct!(Ret);
impl OpExecute for Ret {
    fn execute(&self, c: Components) {
        c.registers.sp -= 1;
        c.registers.pc = c.registers.stack[c.registers.sp as usize];
        c.registers.pc += 2;
    }
}

// Jump to location at nnn
create_opcode_struct_nnn!(Jp);
impl OpExecute for Jp {
    fn execute(&self, c: Components) {
        c.registers.pc = self.nnn;
    }
}

// Jump to location nnn + V0
create_opcode_struct_nnn!(Jp2);
impl OpExecute for Jp2 {
    fn execute(&self, c: Components) {
        c.registers.pc = c.registers.v[0] as u16 + self.nnn;
    }
}

// Call subroutine at nnn
create_opcode_struct_nnn!(Call);
impl OpExecute for Call {
    fn execute(&self, c: Components) {
        c.registers.stack[c.registers.sp as usize] = c.registers.pc;
        c.registers.sp += 1;
        c.registers.pc = self.nnn;
    }
}

// Skip next instruction if Vx == kk
create_opcode_struct_xkk!(SeXkk);
impl OpExecute for SeXkk {
    fn execute(&self, c: Components) {
        if c.registers.v[self.x] == self.kk {
            c.registers.pc += 2;
        }
        c.registers.pc += 2;
    }
}

// Skip next instruction if Vx == Vy
create_opcode_struct_xy!(SeXy);
impl OpExecute for SeXy {
    fn execute(&self, c: Components) {
        if c.registers.v[self.x] == c.registers.v[self.y] {
            c.registers.pc += 2;
        }
        c.registers.pc += 2;
    }
}

// Skip next instruction if Vx != kk
create_opcode_struct_xkk!(SneXkk);
impl OpExecute for SneXkk {
    fn execute(&self, c: Components) {
        if c.registers.v[self.x] != self.kk {
            c.registers.pc += 2;
        }
        c.registers.pc += 2;
    }
}

// Skip next instruction if Vx != Vy
create_opcode_struct_xy!(SneXy);
impl OpExecute for SneXy {
    fn execute(&self, c: Components) {
        if c.registers.v[self.x] != c.registers.v[self.y] {
            c.registers.pc += 2;
        }
        c.registers.pc += 2;
    }
}

// Set Vx == kk
create_opcode_struct_xkk!(LdXkk);
impl OpExecute for LdXkk {
    fn execute(&self, c: Components) {
        c.registers.v[self.x] = self.kk;
        c.registers.pc += 2;
    }
}

// Set Vx = Vy
create_opcode_struct_xy!(LdXy);
impl OpExecute for LdXy {
    fn execute(&self, c: Components) {
        c.registers.v[self.x] = c.registers.v[self.y];
        c.registers.pc += 2;
    }
}

// Set I = nnn
create_opcode_struct_nnn!(LdINnn);
impl OpExecute for LdINnn {
    fn execute(&self, c: Components) {
        c.registers.i = self.nnn;
        c.registers.pc += 2;
    }
}

// Set Vx = delay timer value
create_opcode_struct_x!(LdXDt);
impl OpExecute for LdXDt {
    fn execute(&self, c: Components) {
        c.registers.v[self.x] = c.registers.delay_timer;
        c.registers.pc += 2;
    }
}

// Wait for a key press, store the value of the key in Vx.
create_opcode_struct_x!(LdKey);
impl OpExecute for LdKey {
    fn execute(&self, c: Components) {
        if let Some(key) = c.keyboard.any_key_pressed() {
            c.registers.v[self.x] = key;
            c.registers.pc += 2;
        }
    }
}

// Set delay timer = Vx
create_opcode_struct_x!(LdDtX);
impl OpExecute for LdDtX {
    fn execute(&self, c: Components) {
        c.registers.delay_timer = c.registers.v[self.x];
        c.registers.pc += 2;
    }
}

// Set sound timer = Vx
create_opcode_struct_x!(LdStX);
impl OpExecute for LdStX {
    fn execute(&self, c: Components) {
        c.registers.sound_timer = c.registers.v[self.x];
        c.registers.pc += 2;
        if c.registers.sound_timer > 0 {
            c.audio_device.resume();
        } else {
            c.audio_device.pause();
        }
    }
}

// Set I = location of sprite for digit Vx
create_opcode_struct_x!(LdXSprite);
impl OpExecute for LdXSprite {
    fn execute(&self, c: Components) {
        let val = c.registers.v[self.x] & 0xF;
        // The digit sprites are stored from memory location 0x0 onwards and are
        // 5 bytes long each
        c.registers.i = val as u16 * 0x5;
        c.registers.pc += 2;
    }
}

// Store BCD representation of Vx in memory locations I, I+1, and I+2
create_opcode_struct_x!(LdBcd);
impl OpExecute for LdBcd {
    fn execute(&self, c: Components) {
        let val = c.registers.v[self.x];
        c.memory.write_byte(c.registers.i, val / 100);
        c.memory.write_byte(c.registers.i + 1, val % 100 / 10);
        c.memory.write_byte(c.registers.i + 2, val % 10);
        c.registers.pc += 2;
    }
}

// Store registers V0 through Vx in memory starting at location I
create_opcode_struct_x!(LdIX);
impl OpExecute for LdIX {
    fn execute(&self, c: Components) {
        for (j, val) in (&c.registers.v[..(self.x+1)]).iter().enumerate() {
            c.memory.write_byte(c.registers.i + j as u16, *val);
        }
        c.registers.pc += 2;
    }
}

// Read registers V0 through Vx from memory starting at location I
create_opcode_struct_x!(LdXI);
impl OpExecute for LdXI {
    fn execute(&self, c: Components) {
        for (j, reg) in (&mut c.registers.v[..(self.x+1)]).iter_mut().enumerate() {
            *reg = c.memory.read_byte(c.registers.i + j as u16);
        }
        c.registers.pc += 2;
    }
}

// Set Vx = Vx + kk
create_opcode_struct_xkk!(AddXkk);
impl OpExecute for AddXkk {
    fn execute(&self, c: Components) {
        let vx = &mut c.registers.v[self.x];
        let val1 = Wrapping(*vx);
        let val2 = Wrapping(self.kk);
        *vx = (val1 + val2).0;
        c.registers.pc += 2;
    }
}

// Set Vx = Vx + Vy, set VF = carry
create_opcode_struct_xy!(AddXy);
impl OpExecute for AddXy {
    fn execute(&self, c: Components) {
        let val1 = Wrapping(c.registers.v[self.x]);
        let val2 = Wrapping(c.registers.v[self.y]);
        let sum = val1 + val2;
        let carry = sum < val1;
        c.registers.v[0xF] = carry as u8;
        c.registers.v[self.x] = sum.0;
        c.registers.pc += 2;
    }
}

// Set I = I + Vx
create_opcode_struct_x!(AddIX);
impl OpExecute for AddIX {
    fn execute(&self, c: Components) {
        let vx = Wrapping(c.registers.v[self.x] as u16);
        let i = Wrapping(c.registers.i);
        c.registers.i = (i + vx).0;
        c.registers.pc += 2;
    }
}

// Set Vx = Vx - Vy, set VF = NOT borrow
create_opcode_struct_xy!(Sub);
impl OpExecute for Sub {
    fn execute(&self, c: Components) {
        let vx = Wrapping(c.registers.v[self.x]);
        let vy = Wrapping(c.registers.v[self.y]);
        let difference = vx - vy;
        let borrow = vx < vy;
        c.registers.v[0xF] = !borrow as u8;
        c.registers.v[self.x] = difference.0;
        c.registers.pc += 2;
    }
}

// Set Vx = Vy - Vx, set VF = NOT borrow
create_opcode_struct_xy!(Subn);
impl OpExecute for Subn {
    fn execute(&self, c: Components) {
        let vx = Wrapping(c.registers.v[self.x]);
        let vy = Wrapping(c.registers.v[self.y]);
        let difference = vy - vx;
        let borrow = vy < vx;
        c.registers.v[0xF] = !borrow as u8;
        c.registers.v[self.x] = difference.0;
        c.registers.pc += 2;
    }
}

// Set Vx = Vx OR Vy
create_opcode_struct_xy!(Or);
impl OpExecute for Or {
    fn execute(&self, c: Components) {
        let vy = c.registers.v[self.y];
        let vx = &mut c.registers.v[self.x];
        *vx |= vy;
        c.registers.pc += 2;
    }
}

// Set Vx = Vx AND Vy
create_opcode_struct_xy!(And);
impl OpExecute for And {
    fn execute(&self, c: Components) {
        let vy = c.registers.v[self.y];
        let vx = &mut c.registers.v[self.x];
        *vx &= vy;
        c.registers.pc += 2;
    }
}

// Set Vx = Vx XOR Vy
create_opcode_struct_xy!(Xor);
impl OpExecute for Xor {
    fn execute(&self, c: Components) {
        let vy = c.registers.v[self.y];
        let vx = &mut c.registers.v[self.x];
        *vx ^= vy;
        c.registers.pc += 2;
    }
}

// Set Vx = Vx SHR 1
create_opcode_struct_x!(Shr);
impl OpExecute for Shr {
    fn execute(&self, c: Components) {
        let val = c.registers.v[self.x];
        c.registers.v[0xF] = val % 1;
        c.registers.v[self.x] = val >> 1;
        c.registers.pc += 2;
    }
}

// Set Vx = Vx SHL 1
create_opcode_struct_x!(Shl);
impl OpExecute for Shl {
    fn execute(&self, c: Components) {
        let val = c.registers.v[self.x];
        let msb = (val & 0b1000_0000) > 0;
        c.registers.v[0xF] = msb as u8;
        c.registers.v[self.x] = val << 1;
        c.registers.pc += 2;
    }
}

// Set Vx = random byte AND kk
create_opcode_struct_xkk!(Rnd);
impl OpExecute for Rnd {
    fn execute(&self, c: Components) {
        let mut rng = rand::thread_rng();
        let rand_byte = rng.gen::<u8>();
        let result = rand_byte & self.kk;
        c.registers.v[self.x] = result;
        c.registers.pc += 2;
    }
}

create_opcode_struct_xyn!(Drw);
impl OpExecute for Drw {
    fn execute(&self, c: Components) {
        let x = c.registers.v[self.x];
        let y = c.registers.v[self.y];
        let sprite = c.memory.read_block(c.registers.i, self.n);
        let erased_pixel = c.display.draw_sprite(x, y, sprite);
        c.registers.v[0xF] = erased_pixel as u8;
        c.registers.pc += 2;
    }
}

create_opcode_struct_x!(Skp);
impl OpExecute for Skp {
    fn execute(&self, c: Components) {
        let key = c.registers.v[self.x];
        if c.keyboard.is_pressed(key) {
            c.registers.pc += 2;
        }
        c.registers.pc += 2;
    }
}

create_opcode_struct_x!(Sknp);
impl OpExecute for Sknp {
    fn execute(&self, c: Components) {
        let key = c.registers.v[self.x];
        if !c.keyboard.is_pressed(key) {
            c.registers.pc += 2;
        }
        c.registers.pc += 2;
    }
}

