use crate::commands::command::Command;
use crate::Chip8;

pub struct Return {}

impl Return {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for Return {
    fn execute(&self, emulator: &mut Chip8) {
        emulator.program_counter = emulator
            .stack
            .pop()
            .expect("No value on stack to return to");
    }
}
