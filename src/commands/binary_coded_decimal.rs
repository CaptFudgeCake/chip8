use crate::commands::command::Command;
use crate::Chip8;

pub struct BinaryCodedDecimal {
    register: u8,
}

impl BinaryCodedDecimal {
    pub fn new(register: u8) -> Self {
        Self { register }
    }
}

impl Command for BinaryCodedDecimal {
    fn execute(&self, emulator: &mut Chip8) {
        emulator.memory[emulator.index_register as usize] =
            emulator.registers[self.register as usize] / 100;
        emulator.memory[emulator.index_register as usize + 1] =
            emulator.registers[self.register as usize] % 100 / 10;
        emulator.memory[emulator.index_register as usize + 2] =
            emulator.registers[self.register as usize] % 100 % 10;
    }
}

mod test {
    use crate::Chip8;
    use crate::commands::binary_coded_decimal::BinaryCodedDecimal;
    use crate::commands::command::Command;

    #[test]
    fn test_binary_coded_decimal_htu() {
        let mut emulator = Chip8::new();
        emulator.index_register = 0x200;
        emulator.registers[0] = 235;

        BinaryCodedDecimal::new(0).execute(&mut emulator);

        assert_eq!(emulator.memory[0x200], 2);
        assert_eq!(emulator.memory[0x201], 3);
        assert_eq!(emulator.memory[0x202], 5);
    }

    #[test]
    fn test_binary_coded_decimal_hu() {
        let mut emulator = Chip8::new();
        emulator.index_register = 0x200;
        emulator.registers[0] = 205;

        BinaryCodedDecimal::new(0).execute(&mut emulator);

        assert_eq!(emulator.memory[0x200], 2);
        assert_eq!(emulator.memory[0x201], 0);
        assert_eq!(emulator.memory[0x202], 5);
    }

    #[test]
    fn test_binary_coded_decimal_u() {
        let mut emulator = Chip8::new();
        emulator.index_register = 0x200;
        emulator.registers[0] = 5;

        BinaryCodedDecimal::new(0).execute(&mut emulator);

        assert_eq!(emulator.memory[0x200], 0);
        assert_eq!(emulator.memory[0x201], 0);
        assert_eq!(emulator.memory[0x202], 5);
    }

    #[test]
    fn test_binary_coded_decimal_h() {
        let mut emulator = Chip8::new();
        emulator.index_register = 0x200;
        emulator.registers[0] = 200;

        BinaryCodedDecimal::new(0).execute(&mut emulator);

        assert_eq!(emulator.memory[0x200], 2);
        assert_eq!(emulator.memory[0x201], 0);
        assert_eq!(emulator.memory[0x202], 0);
    }

    #[test]
    fn test_binary_coded_decimal_t() {
        let mut emulator = Chip8::new();
        emulator.index_register = 0x200;
        emulator.registers[0] = 30;

        BinaryCodedDecimal::new(0).execute(&mut emulator);

        assert_eq!(emulator.memory[0x200], 0);
        assert_eq!(emulator.memory[0x201], 3);
        assert_eq!(emulator.memory[0x202], 0);
    }
}