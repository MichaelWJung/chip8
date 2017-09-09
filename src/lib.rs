extern crate rand;
extern crate sdl2;

mod cpu;
mod display;
mod memory;
mod keyboard;

use sdl2::AudioSubsystem;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use std::fs::File;
use std::{thread, time};

pub fn run(file: &mut File) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();
    let audio_device = create_audio_device(&audio_subsystem);

    let window = video_subsystem.window("chip8", 80, 60).position_centered().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let keyboard = keyboard::Keyboard::new(&mut event_pump);
    let memory = memory::BlockMemory::new(file);
    let mut cpu = cpu::Cpu::new(memory, keyboard, audio_device);
    loop {
        for _ in 0..10 {
            cpu.cycle();
        }
        thread::sleep(time::Duration::from_millis(17));
        cpu.decrement_timers();
        cpu.display.redraw();
    }
}

fn create_audio_device(audio_subsystem: &AudioSubsystem) -> AudioDevice<SquareWave> {
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),  // mono
        samples: None       // default sample size
    };
    audio_subsystem.open_playback(None, &desired_spec, |spec| {
        SquareWave {
            phase_inc: 440.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.25
        }
    }).unwrap()
}

pub struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase >= 0.0 && self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
