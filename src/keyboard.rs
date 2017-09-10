use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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

pub struct Keyboard<'a> {
    key_statuses: [bool; 16],
    key_press_pending: bool,
    event_pump: &'a mut EventPump,
}

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
            let first_key = self.key_statuses.iter().position(|&x| x);
            ret = match first_key {
                Some(x) => Some(x as u8),
                None => None,
            };
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
        let key = match keycode {
            KEY_0 => 0x0,
            KEY_1 => 0x1,
            KEY_2 => 0x2,
            KEY_3 => 0x3,
            KEY_4 => 0x4,
            KEY_5 => 0x5,
            KEY_6 => 0x6,
            KEY_7 => 0x7,
            KEY_8 => 0x8,
            KEY_9 => 0x9,
            KEY_A => 0xA,
            KEY_B => 0xB,
            KEY_C => 0xC,
            KEY_D => 0xD,
            KEY_E => 0xE,
            KEY_F => 0xF,
            _ => return,
        };
        self.key_statuses[key] = down;
        self.key_press_pending = true;
    }
}
