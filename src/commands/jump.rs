use crate::commands::command::Command;
use crate::Chip8;

pub struct Jump {
    address: u16,
}

impl Jump {
    pub fn new(address: u16) -> Self {
        Self { address }
    }
}

impl Command for Jump {
    fn execute(&self, emulator: &mut Chip8) {
        emulator.program_counter = self.address;
    }
}
