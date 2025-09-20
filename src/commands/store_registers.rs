use crate::commands::command::Command;
use crate::Chip8;

pub struct StoreRegisters {
    register: u8,
}

impl StoreRegisters {
    pub fn new(register: u8) -> Self {
        Self { register }
    }
}

impl Command for StoreRegisters {
    fn execute(&self, emulator: &mut Chip8) {
        for i in 0..=(self.register as usize) {
            emulator.memory[emulator.index_register as usize + i] = emulator.registers[i];
        }
    }
}

mod test {
    use crate::Chip8;
    use crate::commands::command::Command;
    use crate::commands::store_registers::StoreRegisters;

    #[test]
    fn test_store_registers_in_memory() {
        let mut emulator = Chip8::new();
        emulator.index_register = 0x200;
        emulator.registers[0] = 30;
        emulator.registers[1] = 12;
        emulator.registers[2] = 89;
        emulator.registers[3] = 23;
        emulator.registers[4] = 65;
        emulator.registers[5] = 34;
        emulator.registers[6] = 67;
        emulator.registers[7] = 88;

        StoreRegisters::new(7).execute(&mut emulator);

        assert_eq!(emulator.memory[0x200], 30);
        assert_eq!(emulator.memory[0x201], 12);
        assert_eq!(emulator.memory[0x202], 89);
        assert_eq!(emulator.memory[0x203], 23);
        assert_eq!(emulator.memory[0x204], 65);
        assert_eq!(emulator.memory[0x205], 34);
        assert_eq!(emulator.memory[0x206], 67);
        assert_eq!(emulator.memory[0x207], 88);
    }
}