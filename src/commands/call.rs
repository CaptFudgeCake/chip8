use crate::commands::command::Command;
use crate::Chip8;

pub struct Call {
    address: u16,
}

impl Call {
    pub fn new(address: u16) -> Self {
        Self { address }
    }
}

impl Command for Call {
    fn execute(&self, emulator: &mut Chip8) {
        emulator.stack.push(emulator.program_counter);
        emulator.program_counter = self.address;
    }
}
