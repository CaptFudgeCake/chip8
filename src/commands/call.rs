use crate::commands::command::Command;
use crate::Chip8;

pub struct Call {
    address: u16,
}

impl Call {
    pub fn new(address: u16) -> Self {
        Self { address }
    }
}

impl Command for Call {
    fn execute(&self, emulator: &mut Chip8) {
        emulator.stack.push(emulator.program_counter);
        emulator.program_counter = self.address;
    }
}

mod test {
    use crate::Chip8;
    use crate::commands::call::Call;
    use crate::commands::command::Command;

    #[test]
    fn test_call_function() {
        let mut emulator = Chip8::new();
        emulator.program_counter = 0x200;

        Call::new(0x543).execute(&mut emulator);

        assert_eq!(emulator.stack.len(), 1);
        assert_eq!(emulator.stack[0], 0x200);
        assert_eq!(emulator.program_counter, 0x543);
    }
}