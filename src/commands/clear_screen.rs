use super::command::Command;
use crate::Chip8;

pub struct ClearScreen {}

impl ClearScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for ClearScreen {
    fn execute(&self, emulator: &mut Chip8) {
        for row in emulator.display_data.iter_mut() {
            for pixel in row {
                *pixel = false;
            }
        }
    }
}
