use crate::Chip8;

pub(crate) trait Command {
    fn execute(&self, emulator: &mut Chip8);
}