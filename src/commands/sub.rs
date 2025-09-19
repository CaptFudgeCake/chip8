use crate::Chip8;
use crate::commands::command::Command;

pub struct Sub {
    register_x: u8,
    register_y: u8,
}

impl Sub {
    pub fn new(register_x: u8, register_y: u8) -> Self {
        Self {register_x, register_y}
    }
}

impl Command for Sub {
    fn execute(&self, emulator: &mut Chip8) {
        let (value, overflow) =
            emulator.registers[self.register_x as usize].overflowing_sub(emulator.registers[self.register_y as usize]);
        emulator.registers[self.register_y as usize] = value;
        emulator.registers[0xF] = !overflow as u8;
    }
}