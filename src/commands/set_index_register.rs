use crate::commands::command::Command;
use crate::Chip8;

pub struct SetIndexRegister {
    index: u16,
}

impl SetIndexRegister {
    pub fn new(index: u16) -> Self {
        Self { index }
    }
}

impl Command for SetIndexRegister {
    fn execute(&self, emulator: &mut Chip8) {
        emulator.index_register = self.index;
    }
}

mod test {
    use crate::Chip8;
    use crate::commands::command::Command;
    use crate::commands::set_index_register::SetIndexRegister;

    #[test]
    fn test_set_index_register() {
        let mut emulator = Chip8::new();

        SetIndexRegister::new(0xFFF).execute(&mut emulator);

        assert_eq!(emulator.index_register, 0xFFF);
    }
}