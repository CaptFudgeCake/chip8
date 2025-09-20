use crate::commands::command::Command;
use crate::Chip8;

pub struct Load {
    register_x: u8,
    register_y: u8,
}

impl Load {
    pub fn new(register_x: u8, register_y: u8) -> Self {
        Self {
            register_x,
            register_y,
        }
    }
}

impl Command for Load {
    fn execute(&self, emulator: &mut Chip8) {
        emulator.registers[self.register_x as usize] = emulator.registers[self.register_y as usize]
    }
}

mod test {
    use crate::Chip8;
    use crate::commands::command::Command;
    use crate::commands::load::Load;

    #[test]
    fn test_load_register_to_register() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 0;
        emulator.registers[5] = 70;
        Load::new(0, 5).execute(&mut emulator);

        assert_eq!(emulator.registers[0], 70);
    }
}