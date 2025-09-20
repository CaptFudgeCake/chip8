use crate::commands::command::Command;
use crate::Chip8;

pub struct SubN {
    register_x: u8,
    register_y: u8,
}

impl SubN {
    pub fn new(register_x: u8, register_y: u8) -> Self {
        Self {
            register_x,
            register_y,
        }
    }
}

impl Command for SubN {
    fn execute(&self, emulator: &mut Chip8) {
        let (value, overflow) = emulator.registers[self.register_y as usize]
            .overflowing_sub(emulator.registers[self.register_x as usize]);
        emulator.registers[self.register_x as usize] = value;
        emulator.registers[0xF] = !overflow as u8;
    }
}

mod test {
    use crate::Chip8;
    use crate::commands::command::Command;
    use crate::commands::sub_n::SubN;

    #[test]
    fn test_sub_reverse_registers() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 5;
        emulator.registers[5] = 6;

        SubN::new(0, 5).execute(&mut emulator);

        assert_eq!(emulator.registers[0], 1);
        assert_eq!(emulator.registers[0xF], 1);
    }

    #[test]
    fn test_sub_reverse_registers_borrow() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 6;
        emulator.registers[5] = 5;

        SubN::new(0, 5).execute(&mut emulator);

        assert_eq!(emulator.registers[0], 255);
        assert_eq!(emulator.registers[0xF], 0);
    }
}