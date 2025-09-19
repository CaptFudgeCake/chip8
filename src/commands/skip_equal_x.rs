use crate::Chip8;
use crate::commands::command::Command;

pub struct SkipEqualX {
    register: u8,
    check_value: u8,
}
impl SkipEqualX {
    pub(crate) fn new(register: u8, check_value: u8) -> Self {
        Self {
            register,
            check_value,
        }
    }
}

impl Command for SkipEqualX {
    fn execute(&self, emulator: &mut Chip8) {
        if emulator.registers[self.register as usize] == self.check_value {
            emulator.program_counter += 2
        }
    }
}