# chip8
A simple implementation of a CHIP-8 virtual machine in Rust.

The CHIP-8 programming language is a simple assembly-like language from the mid-seventies. A number of classic games like Pong, Space Invaders, etc. were ported to CHIP-8 and are available in the public domain.

To build, make sure you have libsdl2 installed, then simply run `cargo build --release` in the root directory.

If you would like to try out this implementation, you most certainly want to update the key mappings in src/keyboard.rs as they are currently optimized for the not so common German keyboard layout Neo2.
