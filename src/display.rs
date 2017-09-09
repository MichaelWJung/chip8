use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

const COLS: usize = 64;
const ROWS: usize = 32;
const PIXELS: usize = (COLS * ROWS) as usize;

pub struct Display<'a> {
    pixels: [bool; PIXELS],
    canvas: Canvas<Window>,
    texture: Texture<'a>,
}

impl<'a> Display<'a> {
    pub fn new(canvas: Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>) -> Display<'a> {
        let texture = texture_creator.create_texture_streaming(
            PixelFormatEnum::RGB24, COLS as u32, ROWS as u32).unwrap();
        Display { pixels: [false; PIXELS], canvas, texture }
    }

    pub fn redraw(&mut self) {
        let pixels = &self.pixels;
        self.texture.with_lock(None, |buffer: &mut [u8], _: usize| {
            for (i, &p) in pixels.iter().enumerate() {
                let offset = i * 3;
                let val = p as u8 * 255;
                buffer[offset] = val;
                buffer[offset + 1] = val;
                buffer[offset + 2] = val;
            }
        }).unwrap();
        self.canvas.clear();
        self.canvas.copy(&self.texture, None, None).unwrap();
        self.canvas.present();
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
}

