pub struct Keyboard {
    key_statuses: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard { key_statuses: [false; 16] }
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        false
    }

    pub fn any_key_pressed(&self) -> Option<u8> {
        None
    }
}
