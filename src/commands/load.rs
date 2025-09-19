use crate::Chip8;
use crate::commands::command::Command;

pub struct Load {
    register_x: u8,
    register_y: u8,
}

impl Load {
    pub fn new(register_x: u8, register_y: u8) -> Self {
        Self {register_x,register_y}
    }
}

impl Command for Load {
    fn execute(&self, emulator: &mut Chip8) {
        emulator.registers[self.register_x as usize] = emulator.registers[self.register_y as usize]
    }
}