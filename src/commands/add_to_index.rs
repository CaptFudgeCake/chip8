use crate::commands::command::Command;
use crate::Chip8;

pub struct AddToIndex {
    register: u8,
}

impl AddToIndex {
    pub(crate) fn new(register: u8) -> Self {
        Self { register }
    }
}

impl Command for AddToIndex {
    fn execute(&self, emulator: &mut Chip8) {
        let register_value = emulator.registers[self.register as usize] as u16;
        emulator.index_register = emulator.index_register.wrapping_add(register_value);
    }
}

mod test {
    use crate::Chip8;
    use crate::commands::add_to_index::AddToIndex;
    use crate::commands::command::Command;

    #[test]
    fn test_add_to_index() {
        let mut emulator = Chip8::new();
        emulator.index_register = 0x200;
        emulator.registers[5] = 10;

        AddToIndex::new(5).execute(&mut emulator);

        assert_eq!(emulator.index_register, 0x20A);
    }
}