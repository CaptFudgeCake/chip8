use crate::Chip8;
use crate::commands::command::Command;

pub struct AddToIndex {
    register: u8,
}

impl AddToIndex {
    pub(crate) fn new(register: u8) -> Self {
        Self {
            register,
        }
    }
}

impl Command for AddToIndex {
    fn execute(&self, emulator: &mut Chip8) {
        let register_value = emulator.registers[self.register as usize] as u16;
        emulator.index_register = emulator.index_register.wrapping_add(register_value);
    }
}