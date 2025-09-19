use crate::commands::command::Command;
use crate::Chip8;

pub struct BinaryCodedDecimal {
    register: u8,
}

impl BinaryCodedDecimal {
    pub fn new(register: u8) -> Self {
        Self { register }
    }
}

impl Command for BinaryCodedDecimal {
    fn execute(&self, emulator: &mut Chip8) {
        emulator.memory[emulator.index_register as usize] =
            emulator.registers[self.register as usize] / 100;
        emulator.memory[emulator.index_register as usize + 1] =
            emulator.registers[self.register as usize] % 100 / 10;
        emulator.memory[emulator.index_register as usize + 2] =
            emulator.registers[self.register as usize] % 100 % 10;
    }
}
