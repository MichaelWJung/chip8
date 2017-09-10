use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Keyboard<'a> {
    key_statuses: [bool; 16],
    key_press_pending: bool,
    event_pump: &'a mut EventPump,
}

const KEY_0: Keycode = Keycode::Comma;
const KEY_1: Keycode = Keycode::Num7;
const KEY_2: Keycode = Keycode::Num8;
const KEY_3: Keycode = Keycode::Num9;
const KEY_4: Keycode = Keycode::H;
const KEY_5: Keycode = Keycode::G;
const KEY_6: Keycode = Keycode::F;
const KEY_7: Keycode = Keycode::N;
const KEY_8: Keycode = Keycode::R;
const KEY_9: Keycode = Keycode::T;
const KEY_A: Keycode = Keycode::M;
const KEY_B: Keycode = Keycode::Period;
const KEY_C: Keycode = Keycode::Num0;
const KEY_D: Keycode = Keycode::Q;
const KEY_E: Keycode = Keycode::D;
const KEY_F: Keycode = Keycode::J;


impl<'a> Keyboard<'a> {
    pub fn new(event_pump: &'a mut EventPump) -> Keyboard<'a> {
        Keyboard {
            key_statuses: [false; 16],
            key_press_pending: false,
            event_pump,
        }
    }

    pub fn is_pressed(&mut self, key: u8) -> bool {
        self.key_press_pending = false;
        self.key_statuses[key as usize]
    }

    pub fn any_key_pressed(&mut self) -> Option<u8> {
        let mut ret: Option<u8> = None;
        if self.key_press_pending {
            for i in 0u8..16u8 {
                if self.key_statuses[i as usize] {
                    ret = Some(i);
                    break;
                }
            }
        }
        self.key_press_pending = false;
        ret
    }

    pub fn check_events(&mut self) {
        while let Some(event) = self.event_pump.poll_event() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => ::std::process::exit(0),
                Event::KeyDown { keycode: Some(key), .. } => self.update_key_status(key, true),
                Event::KeyUp { keycode: Some(key), .. } => self.update_key_status(key, false),
                _ => {}
            }
        }
    }

    fn update_key_status(&mut self, keycode: Keycode, down: bool) {
        match keycode {
            KEY_0 => { self.key_statuses[0x0] = down; self.key_press_pending = true; },
            KEY_1 => { self.key_statuses[0x1] = down; self.key_press_pending = true; },
            KEY_2 => { self.key_statuses[0x2] = down; self.key_press_pending = true; },
            KEY_3 => { self.key_statuses[0x3] = down; self.key_press_pending = true; },
            KEY_4 => { self.key_statuses[0x4] = down; self.key_press_pending = true; },
            KEY_5 => { self.key_statuses[0x5] = down; self.key_press_pending = true; },
            KEY_6 => { self.key_statuses[0x6] = down; self.key_press_pending = true; },
            KEY_7 => { self.key_statuses[0x7] = down; self.key_press_pending = true; },
            KEY_8 => { self.key_statuses[0x8] = down; self.key_press_pending = true; },
            KEY_9 => { self.key_statuses[0x9] = down; self.key_press_pending = true; },
            KEY_A => { self.key_statuses[0xA] = down; self.key_press_pending = true; },
            KEY_B => { self.key_statuses[0xB] = down; self.key_press_pending = true; },
            KEY_C => { self.key_statuses[0xC] = down; self.key_press_pending = true; },
            KEY_D => { self.key_statuses[0xD] = down; self.key_press_pending = true; },
            KEY_E => { self.key_statuses[0xE] = down; self.key_press_pending = true; },
            KEY_F => { self.key_statuses[0xF] = down; self.key_press_pending = true; },
            _ => ()
        }
    }
}
