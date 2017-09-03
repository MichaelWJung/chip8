use std::fmt;

const COLS: usize = 64;
const ROWS: usize = 32;
const PIXELS: usize = COLS * ROWS;

pub struct Display {
    pixels: [bool; PIXELS],
}

impl Display {
    pub fn new() -> Display {
        Display { pixels: [false; PIXELS] }
    }

    pub fn redraw(&self) {
        Self::clear_terminal();
        for y in 0..ROWS {
            let b = COLS * y;
            let e = b + COLS;
            let line: String = self.pixels[b..e].iter().map(|&p| {
                if p {
                    '█'
                } else {
                    '.'
                }
            }).collect();
            println!("{}", line);
        }
    }

    pub fn clear(&mut self) {
        self.pixels = [false; PIXELS];
    }

    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        let x = x as usize;
        let y = y as usize;
        let mut erased_pixel = false;
        for (j, line) in sprite.iter().enumerate() {
            for i in 0..8 {
                if line & (0x80 >> i) == 0 {
                    continue;
                }
                let px = (x + i) % COLS;
                let py = (y + j) % ROWS;
                erased_pixel |= self.set_pixel(px, py);
            }
        }
        erased_pixel
    }

    fn set_pixel(&mut self, x: usize, y: usize) -> bool {
        let i = y * COLS + x;
        let was_set = self.pixels[i];
        self.pixels[i] ^= true;
        was_set
    }

    fn clear_terminal() {
        print!("{}[2J", 27 as char);
    }
}

impl fmt::Debug for Display {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.pixels[0].fmt(formatter)
    }
}
