use crate::commands::command::Command;
use crate::Chip8;

pub struct SkipNotEqualXY {
    register_x: u8,
    register_y: u8,
}

impl SkipNotEqualXY {
    pub fn new(register_x: u8, register_y: u8) -> Self {
        Self {
            register_x,
            register_y,
        }
    }
}

impl Command for SkipNotEqualXY {
    fn execute(&self, emulator: &mut Chip8) {
        if emulator.registers[self.register_x as usize]
            != emulator.registers[self.register_y as usize]
        {
            emulator.program_counter += 2
        }
    }
}
