struct Chip8 {
    memory: [u8; 4096],
    display: [[bool; 32]; 64],
    program_counter: u16,
    index_regiser: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    registers: [u8; 16],
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut new_chip8 = Chip8 {
            memory: [0;4096],
            display: [[false;32]; 64],
            program_counter: 0,
            index_regiser: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; 16],
        };

        new_chip8.set_defaults();
        new_chip8
    }

    fn set_defaults(&mut self){
        self.set_fonts();

    }

    fn set_fonts(&mut self) {
        let font = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        for (i, byte) in font.into_iter().enumerate() {
            self.memory[0x050 + i] = byte;
        }
    }

    fn get_command(&self, command: [u8; 2]) -> Chip8Commands {
        let nibble1 = (command[0] & 0xF0) >> 4;
        match nibble1 {
            0 => {
                match command {
                    [0x00, 0xE0] => Chip8Commands::ClearScreen,
                    [0x00, 0xEE] => Chip8Commands::Return,
                    _ => panic!("0NNN command can't be run since it is dependant on specific hardware")
                }
            }
            1 => {
                let address = ((command[0] as u16 & 0xF) << 8) | command[1] as u16;
                Chip8Commands::Jump(address)
            }
            6 => {
                let nibble2 = command[0] & 0xF;
                let value = command[1];
                Chip8Commands::SetRegister(nibble2, value)
            }
            7 => {
                let nibble2 = command[0] & 0xF;
                let value = command[1];
                Chip8Commands::AddValueToRegister(nibble2, value)
            }
            0xA => {
                let address = ((command[0] as u16 & 0xF) << 8) | command[1] as u16;
                Chip8Commands::SetIndexRegister(address)
            }
            0xD => {
                let nibble2 = command[0] & 0xF;
                let nibble3 = (command[1] & 0xF0) >> 4;
                let nibble4 = command[1] & 0xF;
                Chip8Commands::Draw(nibble2, nibble3, nibble4)
            }
            _ => panic!("Command {:x?} not found", command)
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Chip8Commands {
    ClearScreen, // 00E0
    Return, // 00EE
    Jump(u16), // 1NNN
    SetRegister(u8, u8), // 6XNN
    AddValueToRegister(u8, u8), // 7XNN
    SetIndexRegister(u16), // ANNN
    Draw(u8, u8, u8) // DXYN
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod test {
    use crate::{Chip8, Chip8Commands};

    #[test]
    fn test_command_recognition() {
        let emulator = Chip8::new();
        let commands: [[u8;2];7] = [
            [0x00, 0xE0],
            [0x00, 0xEE],
            [0x11, 0x11],
            [0x63, 0x69],
            [0x75, 0x53],
            [0xAD, 0xFF],
            [0xD3, 0x28]
        ];
        let expected = [
            Chip8Commands::ClearScreen,
            Chip8Commands::Return,
            Chip8Commands::Jump(0x111),
            Chip8Commands::SetRegister(3, 0x69),
            Chip8Commands::AddValueToRegister(5, 0x53),
            Chip8Commands::SetIndexRegister(0xDFF),
            Chip8Commands::Draw(3, 2, 8)
        ];

        for (i, command) in commands.into_iter().enumerate() {
            let result = emulator.get_command(command);
            let expected = &expected[i];
    
            assert_eq!(result, *expected)
        };

    }
}