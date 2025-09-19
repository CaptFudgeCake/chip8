use crate::Chip8;
use crate::commands::command::Command;

pub struct AddValueToRegister {
    register: u8,
    value: u8,
}

impl AddValueToRegister {
    pub fn new(register: u8, value: u8) -> Self {
        Self {
            register,
            value
        }
    }
}

impl Command for AddValueToRegister {
    fn execute(&self, emulator: &mut Chip8) {
        (emulator.registers[self.register as usize], _) =
            emulator.registers[self.register as usize].overflowing_add(self.value);
    }
}