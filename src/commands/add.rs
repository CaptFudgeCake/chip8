use crate::commands::command::Command;
use crate::Chip8;

pub struct Add {
    register_x: u8,
    register_y: u8,
}

impl Add {
    pub(crate) fn new(register_x: u8, register_y: u8) -> Self {
        Self {
            register_x,
            register_y,
        }
    }
}

impl Command for Add {
    fn execute(&self, emulator: &mut Chip8) {
        let (value, overflow) = emulator.registers[self.register_x as usize]
            .overflowing_add(emulator.registers[self.register_y as usize]);
        emulator.registers[self.register_x as usize] = value;
        emulator.registers[0xF] = overflow as u8;
    }
}
