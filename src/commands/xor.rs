use crate::commands::command::Command;
use crate::Chip8;

pub struct Xor {
    register_x: u8,
    register_y: u8,
}

impl Xor {
    pub fn new(register_x: u8, register_y: u8) -> Self {
        Self {
            register_x,
            register_y,
        }
    }
}

impl Command for Xor {
    fn execute(&self, emulator: &mut Chip8) {
        emulator.registers[self.register_x as usize] ^=
            emulator.registers[self.register_y as usize];
    }
}

mod test {
    use crate::Chip8;
    use crate::commands::command::Command;
    use crate::commands::xor::Xor;

    #[test]
    fn test_bitwise_xor() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 0b10101111;
        emulator.registers[5] = 0b01010001;

        Xor::new(0, 5).execute(&mut emulator);

        assert_eq!(emulator.registers[0], 0b11111110);
    }
}