extern crate chip8;

use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new("roms/PUZZLE");
    let mut file = File::open(path).unwrap();
    chip8::run(&mut file);
}
