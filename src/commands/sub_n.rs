use crate::Chip8;
use crate::commands::command::Command;

pub struct SubN {
    register_x: u8,
    register_y: u8,
}

impl SubN {
    pub fn new(register_x: u8, register_y: u8) -> Self {
        Self {register_x, register_y}
    }
}

impl Command for SubN {
    fn execute(&self, emulator: &mut Chip8) {
        let (value, overflow) =
            emulator.registers[self.register_y as usize].overflowing_sub(emulator.registers[self.register_x as usize]);
        emulator.registers[self.register_x as usize] = value;
        emulator.registers[0xF] = !overflow as u8;
    }
}