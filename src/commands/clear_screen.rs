use super::command::Command;
use crate::Chip8;

pub struct ClearScreen {}

impl ClearScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for ClearScreen {
    fn execute(&self, emulator: &mut Chip8) {
        for row in emulator.display_data.iter_mut() {
            for pixel in row {
                *pixel = false;
            }
        }
    }
}


mod test {
    use crate::Chip8;
    use crate::commands::clear_screen::ClearScreen;
    use crate::commands::command::Command;

    #[test]
    fn test_clear_screen() {
        let mut emulator = Chip8::new();
        for x in 0..64 {
            for y in 0..32 {
                emulator.display_data[x][y] = true;
            }
        }
    
        ClearScreen::new().execute(&mut emulator);
    
        for row in emulator.display_data {
            for pixel in row {
                assert!(!pixel)
            }
        }
    }
}
