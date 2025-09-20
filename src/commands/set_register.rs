use crate::commands::command::Command;
use crate::Chip8;

pub struct SetRegister {
    register: u8,
    value: u8,
}

impl SetRegister {
    pub fn new(register: u8, value: u8) -> Self {
        Self { register, value }
    }
}

impl Command for SetRegister {
    fn execute(&self, emulator: &mut Chip8) {
        emulator.registers[self.register as usize] = self.value;
    }
}

mod test {
    use crate::Chip8;
    use crate::commands::command::Command;
    use crate::commands::set_register::SetRegister;

    #[test]
    fn test_set_register() {
        let mut emulator = Chip8::new();

        SetRegister::new(2, 69).execute(&mut emulator);

        assert_eq!(emulator.registers[2], 69);
    }
}