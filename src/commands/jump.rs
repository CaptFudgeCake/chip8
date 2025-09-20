use crate::commands::command::Command;
use crate::Chip8;

pub struct Jump {
    address: u16,
}

impl Jump {
    pub fn new(address: u16) -> Self {
        Self { address }
    }
}

impl Command for Jump {
    fn execute(&self, emulator: &mut Chip8) {
        emulator.program_counter = self.address;
    }
}

mod test{
    use crate::Chip8;
    use crate::commands::command::Command;
    use crate::commands::jump::Jump;

    #[test]
    fn test_jump() {
        let mut emulator = Chip8::new();

        Jump::new(0x22A).execute(&mut emulator);

        assert_eq!(emulator.program_counter, 0x22a);
    }
}