use crate::commands::command::Command;
use crate::Chip8;

pub struct SetRegister {
    register: u8,
    value: u8,
}

impl SetRegister {
    pub fn new(register: u8, value: u8) -> Self {
        Self { register, value }
    }
}

impl Command for SetRegister {
    fn execute(&self, emulator: &mut Chip8) {
        emulator.registers[self.register as usize] = self.value;
    }
}
