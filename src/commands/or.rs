use crate::commands::command::Command;
use crate::Chip8;

pub struct Or {
    register_x: u8,
    register_y: u8,
}

impl Or {
    pub fn new(register_x: u8, register_y: u8) -> Self {
        Self {
            register_x,
            register_y,
        }
    }
}

impl Command for Or {
    fn execute(&self, emulator: &mut Chip8) {
        emulator.registers[self.register_x as usize] |=
            emulator.registers[self.register_y as usize];
    }
}
