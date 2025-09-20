use crate::commands::command::Command;
use crate::Chip8;

pub struct AddValueToRegister {
    register: u8,
    value: u8,
}

impl AddValueToRegister {
    pub fn new(register: u8, value: u8) -> Self {
        Self { register, value }
    }
}

impl Command for AddValueToRegister {
    fn execute(&self, emulator: &mut Chip8) {
        (emulator.registers[self.register as usize], _) =
            emulator.registers[self.register as usize].overflowing_add(self.value);
    }
}

mod test {
    use crate::Chip8;
    use crate::commands::add_value_to_register::AddValueToRegister;
    use crate::commands::command::Command;

    #[test]
    fn test_add_to_register() {
        let mut emulator = Chip8::new();

        AddValueToRegister::new(2, 6).execute(&mut emulator);
        AddValueToRegister::new(2, 9).execute(&mut emulator);

        assert_eq!(emulator.registers[2], 15);
    }

    #[test]
    fn test_add_to_register_overflow_shouldnt_fail() {
        let mut emulator = Chip8::new();

        AddValueToRegister::new(2, 129).execute(&mut emulator);
        AddValueToRegister::new(2, 128).execute(&mut emulator);

        assert_eq!(emulator.registers[2], 1);
    }
}
