use crate::commands::command::Command;
use crate::Chip8;

pub struct Return {}

impl Return {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for Return {
    fn execute(&self, emulator: &mut Chip8) {
        emulator.program_counter = emulator
            .stack
            .pop()
            .expect("No value on stack to return to");
    }
}

mod test {
    use crate::Chip8;
    use crate::commands::command::Command;
    use crate::commands::return_command::Return;

    #[test]
    fn test_return() {
        let mut emulator = Chip8::new();
        emulator.stack = vec![0x208];
        emulator.program_counter = 0x500;

        Return::new().execute(&mut emulator);

        assert_eq!(emulator.program_counter, 0x208);
    }
}
