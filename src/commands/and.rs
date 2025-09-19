use crate::commands::command::Command;
use crate::Chip8;

pub struct And {
    register_x: u8,
    register_y: u8,
}

impl And {
    pub fn new(register_x: u8, register_y: u8) -> Self {
        Self {
            register_x,
            register_y,
        }
    }
}

impl Command for And {
    fn execute(&self, emulator: &mut Chip8) {
        emulator.registers[self.register_x as usize] &=
            emulator.registers[self.register_y as usize];
    }
}
