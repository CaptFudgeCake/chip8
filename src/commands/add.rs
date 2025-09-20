use crate::commands::command::Command;
use crate::Chip8;

pub struct Add {
    register_x: u8,
    register_y: u8,
}

impl Add {
    pub(crate) fn new(register_x: u8, register_y: u8) -> Self {
        Self {
            register_x,
            register_y,
        }
    }
}

impl Command for Add {
    fn execute(&self, emulator: &mut Chip8) {
        let (value, overflow) = emulator.registers[self.register_x as usize]
            .overflowing_add(emulator.registers[self.register_y as usize]);
        emulator.registers[self.register_x as usize] = value;
        emulator.registers[0xF] = overflow as u8;
    }
}

mod test {
    use crate::Chip8;
    use crate::commands::add::Add;
    use crate::commands::command::Command;

    #[test]
    fn test_add_registers() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 6;
        emulator.registers[5] = 5;

        Add::new(0, 5).execute(&mut emulator);

        assert_eq!(emulator.registers[0], 11);
    }

    #[test]
    fn test_add_registers_overflow() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 255;
        emulator.registers[5] = 5;

        Add::new(0, 5).execute(&mut emulator);

        assert_eq!(emulator.registers[0], 4);
        assert_eq!(emulator.registers[0xF], 1);
    }
}