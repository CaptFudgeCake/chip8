use crate::commands::command::Command;
use crate::Chip8;

pub struct SkipNotEqualX {
    register: u8,
    check_value: u8,
}

impl SkipNotEqualX {
    pub fn new(register: u8, check_value: u8) -> Self {
        Self {
            register,
            check_value,
        }
    }
}

impl Command for SkipNotEqualX {
    fn execute(&self, emulator: &mut Chip8) {
        if emulator.registers[self.register as usize] != self.check_value {
            emulator.program_counter += 2
        }
    }
}

mod test {
    use crate::Chip8;
    use crate::commands::command::Command;
    use crate::commands::skip_not_equal_x::SkipNotEqualX;

    #[test]
    fn test_not_skip_equal_to_register() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 6;
        emulator.program_counter = 0x200;
        SkipNotEqualX::new(0, 0x7).execute(&mut emulator);

        assert_eq!(emulator.program_counter, 0x202)
    }

    #[test]
    fn test_skip_not_equal_to_register_equal() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 7;
        emulator.program_counter = 0x200;
        SkipNotEqualX::new(0, 0x7).execute(&mut emulator);

        assert_eq!(emulator.program_counter, 0x200)
    }
}