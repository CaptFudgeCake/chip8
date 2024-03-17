use std::{fs::File, io::{self, stdout, Read, Write}, thread, time::{self, Instant}};

use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};

struct Chip8 {
    memory: [u8; 4096],
    display: [[bool; 32]; 64],
    program_counter: u16,
    index_regiser: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    registers: [u8; 16],
    stdout: io::Stdout
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut new_chip8 = Chip8 {
            memory: [0; 4096],
            display: [[false; 32]; 64],
            program_counter: 0x200,
            index_regiser: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; 16],
            stdout: stdout()
        };

        new_chip8.set_defaults();
        new_chip8
    }

    fn set_defaults(&mut self) {
        self.set_fonts();
        let _ = self.stdout.execute(cursor::Hide);
    }

    pub fn load_program(&mut self, program: &[u8]) {
        let mut address = 0x200;
        if (address + program.len()) > 4096 {panic!("Program too long to fit in ram, solution not implemented")}

        for byte in program {
            self.memory[address] = *byte;
            address += 1;
        }
    }

    pub fn start(&mut self){
        let target_ft = time::Duration::from_secs(1) / 700;
        // let start = time::Instant::now();
        loop {
            let now = time::Instant::now();
            let command = &self.memory[(self.program_counter as usize)..(self.program_counter as usize +2)];
            self.program_counter += 2;
            let decoded_command = self.get_command(command);
            self.execute_command(decoded_command);
            self.draw_display().expect("Error drawing to screen");
            if let Some(i) = target_ft.checked_sub(now.elapsed()) {
                thread::sleep(i);
            }
        }
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
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        for (i, byte) in font.into_iter().enumerate() {
            self.memory[0x050 + i] = byte;
        }
    }

    fn execute_command(&mut self, command: Chip8Commands) {
        match command {
            Chip8Commands::ClearScreen => {
                for row in self.display.iter_mut(){
                    for pixel in row {
                        *pixel = false;
                    }
                }
            },
            Chip8Commands::Return => todo!(),
            Chip8Commands::Jump(address) => {
                self.program_counter = address;
            },
            Chip8Commands::SetRegister(register, value) => {
                self.registers[register] = value;
            },
            Chip8Commands::AddValueToRegister(register, value) => {
                self.registers[register] += value;
            },
            Chip8Commands::SetIndexRegister(value) => {
                self.index_regiser = value;
            },
            Chip8Commands::Draw(x, y, bytes) => {
                for byte_offset in 0..=bytes {
                    let byte = self.memory[self.index_regiser as usize + byte_offset as usize];
                    for i in 0..8 {
                        let bit = byte >> i;
                        let test = (bit & 0b1) != 0;
                        let x_pos = (x + i) % 64;
                        let y_pos = (y + byte_offset as usize) % 32;
                        if self.display[x_pos][y_pos] {
                            self.registers[0xF] = 1;
                        }
                        self.display[x_pos][y_pos] ^= test;
                    }
                }
            },
        }
    }

    fn get_command(&self, command: &[u8]) -> Chip8Commands {
        let nibble1 = (command[0] & 0xF0) >> 4;
        match nibble1 {
            0 => match command {
                [0x00, 0xE0] => Chip8Commands::ClearScreen,
                [0x00, 0xEE] => Chip8Commands::Return,
                _ => panic!("0NNN command can't be run since it is dependant on specific hardware"),
            },
            1 => {
                let address = ((command[0] as u16 & 0xF) << 8) | command[1] as u16;
                Chip8Commands::Jump(address)
            }
            6 => {
                let nibble2 = command[0] & 0xF;
                let value = command[1];
                Chip8Commands::SetRegister(nibble2.into(), value)
            }
            7 => {
                let nibble2 = command[0] & 0xF;
                let value = command[1];
                Chip8Commands::AddValueToRegister(nibble2.into(), value)
            }
            0xA => {
                let address = ((command[0] as u16 & 0xF) << 8) | command[1] as u16;
                Chip8Commands::SetIndexRegister(address)
            }
            0xD => {
                let nibble2 = command[0] & 0xF;
                let nibble3 = (command[1] & 0xF0) >> 4;
                let nibble4 = command[1] & 0xF;
                Chip8Commands::Draw(nibble2.into(), nibble3.into(), nibble4)
            }
            _ => panic!("Command {:x?} not found", command),
        }
    }

    fn draw_display(&mut self) -> io::Result<()>{
        self.stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        for y in 0..32 {
            for x in 0..64 {
                if self.display[x][y] {
                    self.stdout
                        .queue(cursor::MoveTo(x.try_into().unwrap(), y.try_into().unwrap()))?
                        .queue(style::PrintStyledContent("█".white()))?;
                } else {
                    self.stdout
                        .queue(cursor::MoveTo(x.try_into().unwrap(), y.try_into().unwrap()))?
                        .queue(style::PrintStyledContent("█".hidden()))?;
                }
            }
        }
        self.stdout.flush()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Chip8Commands {
    ClearScreen,                // 00E0
    Return,                     // 00EE
    Jump(u16),                  // 1NNN
    SetRegister(usize, u8),        // 6XNN
    AddValueToRegister(usize, u8), // 7XNN
    SetIndexRegister(u16),      // ANNN
    Draw(usize, usize, u8),           // DXYN
}

fn main() {
    let mut program: Vec<u8> = Vec::new();
    let mut file = File::open("IBM Logo.ch8").unwrap(); 
    file.read_to_end(&mut program).expect("Failed to read program");
    let mut emulator = Chip8::new();
    emulator.load_program(&program);
    emulator.start();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_command_decode() {
        let emulator = Chip8::new();
        let commands: [[u8; 2]; 7] = [
            [0x00, 0xE0],
            [0x00, 0xEE],
            [0x11, 0x11],
            [0x63, 0x69],
            [0x75, 0x53],
            [0xAD, 0xFF],
            [0xD3, 0x28],
        ];
        let expected = [
            Chip8Commands::ClearScreen,
            Chip8Commands::Return,
            Chip8Commands::Jump(0x111),
            Chip8Commands::SetRegister(3, 0x69),
            Chip8Commands::AddValueToRegister(5, 0x53),
            Chip8Commands::SetIndexRegister(0xDFF),
            Chip8Commands::Draw(3, 2, 8),
        ];

        for (i, command) in commands.into_iter().enumerate() {
            let result = emulator.get_command(&command);
            let expected = &expected[i];

            assert_eq!(result, *expected)
        }
    }

    #[test]
    fn test_draw_command() {
        let mut emulator = Chip8::new();
        emulator.memory[0x200] = 0xFF;
        emulator.memory[0x201] = 0xFF;
        emulator.memory[0x202] = 0xFF;
        emulator.memory[0x203] = 0xFF;
        emulator.index_regiser = 0x200;

        let command = Chip8Commands::Draw(3, 2, 4);

        emulator.execute_command(command);
        
        for x in 3..(3+8) {
            for y in 2..(2+4) {
                assert!(emulator.display[x][y], "pixel {}, {} not set correctly", x, y);
            }
        }
    }

    #[test]
    fn test_draw_ovewrite(){
        let mut emulator = Chip8::new();
        emulator.memory[0x200] = 0xFF;
        emulator.memory[0x201] = 0xFF;
        emulator.memory[0x202] = 0xFF;
        emulator.memory[0x203] = 0xFF;
        emulator.index_regiser = 0x200;
        emulator.display[3][2] = true;

        let command = Chip8Commands::Draw(3, 2, 4);

        emulator.execute_command(command);
        
        for x in 3..(3+8) {
            for y in 2..(2+4) {
                if x == 3 && y == 2 {
                    assert!(!emulator.display[x][y], "pixel {}, {} not set correctly", x, y);
                } else {
                    assert!(emulator.display[x][y], "pixel {}, {} not set correctly", x, y);
                }
            }
        }
        assert_eq!(emulator.registers[0xF], 1)
    }
}
