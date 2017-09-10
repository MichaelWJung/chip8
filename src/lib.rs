extern crate rand;
extern crate sdl2;

mod audio;
mod cpu;
mod display;
mod memory;
mod keyboard;

use std::fs::File;
use std::{thread, time};

pub fn run(file: &mut File) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();
    let audio_device = audio::create_audio_device(&audio_subsystem);

    let mut display_context = display::DisplayContext::new(&video_subsystem);
    let mut event_pump = sdl_context.event_pump().unwrap();

    let display = display::Display::new(&mut display_context);
    let keyboard = keyboard::Keyboard::new(&mut event_pump);
    let memory = memory::BlockMemory::new(file);
    let mut cpu = cpu::Cpu::new(memory, display, keyboard, audio_device);
    loop {
        for _ in 0..10 {
            cpu.cycle();
        }
        thread::sleep(time::Duration::from_millis(17));
        cpu.decrement_timers();
        cpu.display.redraw();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
