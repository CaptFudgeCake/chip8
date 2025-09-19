use crate::Chip8;
use crate::commands::command::Command;

pub struct ReadIntoRegisters {
    register: u8,
}

impl ReadIntoRegisters {
    pub fn new(register: u8) -> Self {
        Self {register}
    }
}

impl Command for ReadIntoRegisters {
    fn execute(&self, emulator: &mut Chip8) {
        for i in 0..=(self.register as usize) {
            emulator.registers[i] = emulator.memory[emulator.index_register as usize + i];
        }
    }
}