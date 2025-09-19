use crate::commands::command::Command;
use crate::Chip8;

pub struct Draw {
    register_x: u8,
    register_y: u8,
    bytes: u8,
}

impl Draw {
    pub fn new(register_x: u8, register_y: u8, bytes: u8) -> Self {
        Self {
            register_x,
            register_y,
            bytes,
        }
    }
}

impl Command for Draw {
    fn execute(&self, emulator: &mut Chip8) {
        let x_start = (emulator.registers[self.register_x as usize] as usize) % 64;
        let y_start = (emulator.registers[self.register_y as usize] as usize) % 32;
        for byte_offset in 0..self.bytes {
            let byte = emulator.memory[emulator.index_register as usize + byte_offset as usize];
            for i in 0..8 {
                let bit = ((byte >> 7 - i) & 0b1) != 0;
                let x_pos = x_start + i;
                let y_pos = y_start + byte_offset as usize;
                if x_pos < 64 && y_pos < 32 {
                    if emulator.display_data[x_pos][y_pos] != bit {
                        emulator.registers[0xF] = 1;
                    }
                    emulator.display_data[x_pos][y_pos] ^= bit;
                }
            }
        }

        emulator.display_changed = true;
    }
}
