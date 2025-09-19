use crate::commands::command::Command;
use crate::Chip8;

pub struct ShiftRight {
    register_x: u8,
    register_y: u8,
}

impl ShiftRight {
    pub fn new(register_x: u8, register_y: u8) -> Self {
        Self {
            register_x,
            register_y,
        }
    }
}

impl Command for ShiftRight {
    fn execute(&self, emulator: &mut Chip8) {
        if emulator.use_old_bit_shift {
            emulator.registers[self.register_x as usize] =
                emulator.registers[self.register_y as usize] >> 1;
            emulator.registers[0xF] = emulator.registers[self.register_y as usize] & 0b1;
        } else {
            emulator.registers[0xF] = emulator.registers[self.register_x as usize] & 0b1;
            emulator.registers[self.register_x as usize] =
                emulator.registers[self.register_x as usize] >> 1;
        }
    }
}
