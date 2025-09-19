use crate::Chip8;
use crate::commands::command::Command;

pub struct StoreRegisters {
    register: u8,
}

impl StoreRegisters {
    pub fn new(register: u8) -> Self {
        Self {register}
    }
}

impl Command for StoreRegisters {
    fn execute(&self, emulator: &mut Chip8) {
        for i in 0..=(self.register as usize) {
            emulator.memory[emulator.index_register as usize + i] = emulator.registers[i];
        }
    }
}