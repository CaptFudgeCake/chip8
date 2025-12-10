use crate::Chip8;
use crate::commands::command::Command;

pub struct SetRegisterFromDelay {
    x: u8,
}

impl SetRegisterFromDelay {
    pub fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Command for SetRegisterFromDelay {
    fn execute(&self, emulator: &mut Chip8) {
        emulator.registers[self.x as usize] = emulator.delay_timer;
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute() {
        let mut emulator = Chip8::new();
        emulator.registers[1] = 5;
        emulator.delay_timer = 100;

        let command = SetRegisterFromDelay::new(1);

        command.execute(&mut emulator);

        assert_eq!(emulator.registers[1], 100);
    }
}