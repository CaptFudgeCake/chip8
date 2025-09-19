use crate::commands::command::Command;
use crate::Chip8;

pub struct ShiftLeft {
    register_x: u8,
    register_y: u8,
}

impl ShiftLeft {
    pub fn new(register_x: u8, register_y: u8) -> Self {
        Self {
            register_x,
            register_y,
        }
    }
}

impl Command for ShiftLeft {
    fn execute(&self, emulator: &mut Chip8) {
        if emulator.use_old_bit_shift {
            emulator.registers[self.register_x as usize] =
                emulator.registers[self.register_y as usize] << 1;
            emulator.registers[0xF] = (emulator.registers[self.register_y as usize] & 0x80) >> 7;
        } else {
            emulator.registers[0xF] = (emulator.registers[self.register_x as usize] & 0x80) >> 7;
            emulator.registers[self.register_x as usize] =
                emulator.registers[self.register_x as usize] << 1;
        }
    }
}
