use crate::Chip8;
use crate::commands::command::Command;

pub struct SetDelayTimer {
    x: u8,
}

impl SetDelayTimer {
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Command for SetDelayTimer {
    fn execute(&self, emulator: &mut Chip8) {
            emulator.delay_timer = emulator.registers[self.x as usize];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute() {
        let mut emulator = Chip8::new();
        emulator.registers[1] = 5;

        let command = SetDelayTimer::new(1);

        command.execute(&mut emulator);

        assert_eq!(emulator.delay_timer, 5);
    }
}