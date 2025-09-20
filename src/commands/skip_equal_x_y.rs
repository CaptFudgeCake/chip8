use crate::commands::command::Command;
use crate::Chip8;

pub struct SkipEqualXY {
    register_x: u8,
    register_y: u8,
}

impl SkipEqualXY {
    pub fn new(register_x: u8, register_y: u8) -> Self {
        Self {
            register_x,
            register_y,
        }
    }
}

impl Command for SkipEqualXY {
    fn execute(&self, emulator: &mut Chip8) {
        if emulator.registers[self.register_x as usize]
            == emulator.registers[self.register_y as usize]
        {
            emulator.program_counter += 2
        }
    }
}

mod test {
    use crate::Chip8;
    use crate::commands::command::Command;
    use crate::commands::skip_equal_x_y::SkipEqualXY;

    #[test]
    fn test_not_equal_to_registers() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 6;
        emulator.registers[1] = 6;
        emulator.program_counter = 0x200;
        SkipEqualXY::new(0, 1).execute(&mut emulator);

        assert_eq!(emulator.program_counter, 0x202)
    }

    #[test]
    fn test_skip_equal_to_registers_not_equal() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 7;
        emulator.registers[1] = 6;
        emulator.program_counter = 0x200;
        SkipEqualXY::new(0, 1).execute(&mut emulator);

        assert_eq!(emulator.program_counter, 0x200)
    }
}
