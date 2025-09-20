use crate::commands::command::Command;
use crate::Chip8;

pub struct SkipEqualX {
    register: u8,
    check_value: u8,
}
impl SkipEqualX {
    pub(crate) fn new(register: u8, check_value: u8) -> Self {
        Self {
            register,
            check_value,
        }
    }
}

impl Command for SkipEqualX {
    fn execute(&self, emulator: &mut Chip8) {
        if emulator.registers[self.register as usize] == self.check_value {
            emulator.program_counter += 2
        }
    }
}

mod test {
    use crate::Chip8;
    use crate::commands::command::Command;
    use crate::commands::skip_equal_x::SkipEqualX;

    #[test]
    fn test_skip_equal_to_register() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 6;
        emulator.program_counter = 0x200;
        let command = SkipEqualX::new(0, 0x6);

        command.execute(&mut emulator);

        assert_eq!(emulator.program_counter, 0x202)
    }

    #[test]
    fn test_skip_equal_to_register_not_equal() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 7;
        emulator.program_counter = 0x200;
        let command = SkipEqualX::new(0, 0x6);

        command.execute(&mut emulator);

        assert_eq!(emulator.program_counter, 0x200)
    }
}