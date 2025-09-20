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

mod test {
    use crate::Chip8;
    use crate::commands::command::Command;
    use crate::commands::shift_left::ShiftLeft;

    #[test]
    fn test_shift_left_bit_1() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 0xFF;
        ShiftLeft::new(0, 5).execute(&mut emulator);

        assert_eq!(emulator.registers[0], 0xFE);
        assert_eq!(emulator.registers[0xF], 1);
    }

    #[test]
    fn test_shift_left_bit_0() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 0x7F;

        ShiftLeft::new(0, 5).execute(&mut emulator);

        assert_eq!(emulator.registers[0], 0xFE);
        assert_eq!(emulator.registers[0xF], 0);
    }

    #[test]
    fn test_shift_left_bit_1_vy_used() {
        let mut emulator = Chip8::new();
        emulator.use_old_bit_shift = true;
        emulator.registers[5] = 0xFF;

        ShiftLeft::new(0, 5).execute(&mut emulator);

        assert_eq!(emulator.registers[0], 0xFE);
        assert_eq!(emulator.registers[0xF], 1);
    }

    #[test]
    fn test_shift_left_bit_0_vy_used() {
        let mut emulator = Chip8::new();
        emulator.use_old_bit_shift = true;
        emulator.registers[5] = 0x7F;

        ShiftLeft::new(0, 5).execute(&mut emulator);

        assert_eq!(emulator.registers[0], 0xFE);
        assert_eq!(emulator.registers[0xF], 0);
    }
}
