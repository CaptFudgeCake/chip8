use crate::commands::command::Command;
use crate::Chip8;

pub struct ReadIntoRegisters {
    register: u8,
}

impl ReadIntoRegisters {
    pub fn new(register: u8) -> Self {
        Self { register }
    }
}

impl Command for ReadIntoRegisters {
    fn execute(&self, emulator: &mut Chip8) {
        for i in 0..=(self.register as usize) {
            emulator.registers[i] = emulator.memory[emulator.index_register as usize + i];
        }
    }
}

mod test {
    use crate::Chip8;
    use crate::commands::command::Command;
    use crate::commands::read_into_registers::ReadIntoRegisters;

    #[test]
    fn test_command_read_into_registers() {
        let mut emulator = Chip8::new();
        emulator.index_register = 0x200;
        emulator.registers[0] = 0;
        emulator.registers[1] = 0;

        emulator.memory[0x200] = 20;
        emulator.memory[0x201] = 21;

        ReadIntoRegisters::new(1).execute(&mut emulator);

        assert_eq!(emulator.registers[0], 20);
        assert_eq!(emulator.registers[1], 21);
    }
}