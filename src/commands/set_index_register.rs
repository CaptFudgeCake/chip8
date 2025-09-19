use crate::commands::command::Command;
use crate::Chip8;

pub struct SetIndexRegister {
    index: u16,
}

impl SetIndexRegister {
    pub fn new(index: u16) -> Self {
        Self { index }
    }
}

impl Command for SetIndexRegister {
    fn execute(&self, emulator: &mut Chip8) {
        emulator.index_register = self.index;
    }
}
