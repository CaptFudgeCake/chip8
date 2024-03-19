use std::{fs::File, io::{self, stdout, Read, Write}, thread, time::{self, Instant}, usize};

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
    stdout: io::Stdout,
    display_changed: bool,
    use_old_bit_shift: bool
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
            stdout: stdout(),
            display_changed: false,
            use_old_bit_shift: false
        };

        new_chip8.set_defaults();
        new_chip8
    }

    fn set_defaults(&mut self) {
        self.set_fonts();
        let _ = self.stdout.execute(cursor::Hide);
        let _ = self.stdout.execute(terminal::Clear(terminal::ClearType::All));
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
            let decoded_command = self.decode_command(command);
            self.execute_command(decoded_command);
            if self.display_changed {
                self.draw_display().expect("Failed to draw display to console");
                self.display_changed = false;
            }
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
            Chip8Commands::Return => {
                self.program_counter = self.stack.pop().expect("No value on stack to return to");
            },
            Chip8Commands::Jump(address) => {
                self.program_counter = address;
            },
            Chip8Commands::SetRegister(register, value) => {
                self.registers[register as usize] = value;
            },
            Chip8Commands::AddValueToRegister(register, value) => {
                self.registers[register as usize] += value;
            },
            Chip8Commands::SetIndexRegister(value) => {
                self.index_regiser = value;
            },
            Chip8Commands::Draw(x, y, bytes) => {
                let x_start = (self.registers[x as usize] as usize) % 64;
                let y_start = (self.registers[y as usize] as usize) % 32;
                for byte_offset in 0..bytes {
                    let byte = self.memory[self.index_regiser as usize + byte_offset as usize];
                    for i in 0..8 {
                        let bit = ((byte >> 7-i) & 0b1) != 0;
                        let x_pos = x_start + i;
                        let y_pos = y_start + byte_offset as usize;
                        if x_pos < 64 && y_pos < 32 {
                            if self.display[x_pos][y_pos] != bit {
                                self.registers[0xF] = 1;
                            }
                            self.display[x_pos][y_pos] ^= bit;
                        }
                    }
                }

                self.display_changed = true;
            },
            Chip8Commands::SkipEqualX(x, value) => {
                if self.registers[x as usize] == value {
                    self.program_counter += 2
                }
            },
            Chip8Commands::SkipNotEqualX(x, value) => {
                if self.registers[x as usize] != value {
                    self.program_counter += 2
                }
            },
            Chip8Commands::SkipEqualXY(x, y) => {
                if self.registers[x as usize] == self.registers[y as usize] {
                    self.program_counter += 2
                }
            },
            Chip8Commands::Load(x, value) => {self.registers[x as usize] = value},
            Chip8Commands::OR(_, _) => todo!(),
            Chip8Commands::AND(_, _) => todo!(),
            Chip8Commands::XOR(_, _) => todo!(),
            Chip8Commands::ADD(_, _) => todo!(),
            Chip8Commands::SUB(_, _) => todo!(),
            Chip8Commands::ShiftRight(_, _) => todo!(),
            Chip8Commands::ShiftLeft(_, _) => todo!(),
            Chip8Commands::SkipNotEqualXY(x, y) => {
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.program_counter += 2
                }
            },
            Chip8Commands::BinaryCodedDecimal(_) => todo!(),
            Chip8Commands::StoreRegisters(_) => todo!(),
        }
    }

    fn decode_command(&self, command: &[u8]) -> Chip8Commands {
        let instruction_id = (command[0] & 0xF0) >> 4;
        match instruction_id {
            0 => match command {
                [0x00, 0xE0] => Chip8Commands::ClearScreen,
                [0x00, 0xEE] => Chip8Commands::Return,
                _ => panic!("0NNN command can't be run since it is dependant on specific hardware"),
            },
            3 | 4 | 6 | 7 => {
                let x = command[0] & 0xF;
                match instruction_id {
                    3 => Chip8Commands::SkipEqualX(x.into(), command[1]),
                    4 => Chip8Commands::SkipNotEqualX(x.into(), command[1]),
                    6 => Chip8Commands::SetRegister(x.into(), command[1]),
                    7 => Chip8Commands::AddValueToRegister(x.into(), command[1]),
                    _ => panic!("Instruction {:x?} not found", command),
                }
            }
            8 => {
                let x = command[0] & 0xF;
                let y = (command[1] >> 4) & 0xF;
                let nibble4 = command[1] & 0xF;
                match nibble4 {
                    0x0 => Chip8Commands::Load(x.into(), y.into()),
                    0x1 => Chip8Commands::OR(x.into(), y.into()),
                    0x2 => Chip8Commands::AND(x.into(), y.into()),
                    0x3 => Chip8Commands::XOR(x.into(), y.into()),
                    0x4 => Chip8Commands::ADD(x.into(), y.into()),
                    0x5 => Chip8Commands::SUB(x.into(), y.into()),
                    0x6 => Chip8Commands::ShiftRight(x.into(), y.into()),
                    0xE => Chip8Commands::ShiftLeft(x.into(), y.into()),
                    _ => panic!("Instruction {:x?} not found", command)
                }
            }
            0xF => {
                let x = command[0] & 0xF;
                match command[1] {
                    0x33 => Chip8Commands::BinaryCodedDecimal(x.into()),
                    0x55 => Chip8Commands::StoreRegisters(x.into()),
                    _ => panic!("Instruction {:x?} not found", command)
                }
            }
            1 | 0xA => {
                let address = ((command[0] as u16 & 0xF) << 8) | command[1] as u16;
                match instruction_id {
                    1 => Chip8Commands::Jump(address),
                    0xA => Chip8Commands::SetIndexRegister(address),
                    _ => panic!("Instruction {:x?} not found", command)
                }
            }
            5 => {
                let x = command[0] & 0xF;
                let y = (command[1] >> 4) & 0xF;
                Chip8Commands::SkipEqualXY(x.into(), y.into())
            }
            9 => {
                let x = command[0] & 0xF;
                let y = (command[1] >> 4) & 0xF;
                Chip8Commands::SkipNotEqualXY(x.into(), y.into())
            }
            0xD => {
                let x = command[0] & 0xF;
                let y = (command[1] & 0xF0) >> 4;
                let bytes = command[1] & 0xF;
                Chip8Commands::Draw(x.into(), y.into(), bytes)
            }
            _ => panic!("Instruction {:x?} not found", command),
        }
    }

    fn draw_display(&mut self) -> io::Result<()>{
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
    SkipEqualX(u8, u8),         // 3XNN
    SkipNotEqualX(u8, u8),      // 4XNN	
    SkipEqualXY(u8, u8),        // 5XY0	
    SetRegister(u8, u8),        // 6XNN
    AddValueToRegister(u8, u8), // 7XNN
    Load(u8, u8),               // 8XY0	
    OR(u8, u8),                 // 8XY1	
    AND(u8, u8),                // 8XY2	
    XOR(u8, u8),                // 8XY3	
    ADD(u8, u8),                // 8XY4	
    SUB(u8, u8),                // 8XY5
    ShiftRight(u8, u8),         // 8XY6
    ShiftLeft(u8, u8),          // 8XYE
    SkipNotEqualXY(u8, u8),     // 9XY0	
    SetIndexRegister(u16),      // ANNN
    Draw(u8, u8, u8),           // DXYN
    BinaryCodedDecimal(u8),     // FX33
    StoreRegisters(u8),         // FX55
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
        let commands: [[u8; 2]; 21] = [
            [0x00, 0xE0],
            [0x00, 0xEE],
            [0x11, 0x11],
            [0x63, 0x69],
            [0x75, 0x53],
            [0xAD, 0xFF],
            [0xD3, 0x28],
            [0x30, 0x05],
            [0x41, 0x45],
            [0x5A, 0xD0],
            [0x87, 0xA0],
            [0x89, 0x21],
            [0x8A, 0x32],
            [0x8B, 0x43],
            [0x8C, 0x54],
            [0x8D, 0x65],
            [0x8E, 0x76],
            [0x8F, 0x8E],
            [0x90, 0x90],
            [0xF3, 0x33],
            [0xF6, 0x55]
        ];
        let expected = [
            Chip8Commands::ClearScreen,
            Chip8Commands::Return,
            Chip8Commands::Jump(0x111),
            Chip8Commands::SetRegister(3, 0x69),
            Chip8Commands::AddValueToRegister(5, 0x53),
            Chip8Commands::SetIndexRegister(0xDFF),
            Chip8Commands::Draw(3, 2, 8),
            Chip8Commands::SkipEqualX(0, 5),          
            Chip8Commands::SkipNotEqualX(1, 0x45),       
            Chip8Commands::SkipEqualXY(0xA, 0xD),      
            Chip8Commands::Load(7, 0xA),             
            Chip8Commands::OR(9, 2),               
            Chip8Commands::AND(0xA, 3),              
            Chip8Commands::XOR(0xB, 4),              
            Chip8Commands::ADD(0xC, 5),              
            Chip8Commands::SUB(0xD, 6),              
            Chip8Commands::ShiftRight(0xE, 7),       
            Chip8Commands::ShiftLeft(0xF, 8),        
            Chip8Commands::SkipNotEqualXY(0x0, 9),   
            Chip8Commands::BinaryCodedDecimal(3),      
            Chip8Commands::StoreRegisters(6),          
        ];

        for (i, command) in commands.into_iter().enumerate() {
            let result = emulator.decode_command(&command);
            let expected = &expected[i];

            assert_eq!(result, *expected)
        }
    }

    #[test]
    fn test_clear_screen() {
        let mut emulator = Chip8::new();
        for x in 0..64 {
            for y in 0..32 {
                emulator.display[x][y] = true;
            }
        }

        emulator.execute_command(Chip8Commands::ClearScreen);

        for row in emulator.display {
            for pixel in row {
                assert!(!pixel)
            }
        }
    }

    #[test]
    fn test_jump(){
        let mut emulator = Chip8::new();

        emulator.execute_command(Chip8Commands::Jump(0x22A));

        assert_eq!(emulator.program_counter, 0x22a);
    }

    #[test]
    fn test_add_to_register(){
        let mut emulator = Chip8::new();

        emulator.execute_command(Chip8Commands::AddValueToRegister(2, 6));
        emulator.execute_command(Chip8Commands::AddValueToRegister(2, 9));

        assert_eq!(emulator.registers[2], 15);
    }

    #[test]
    fn test_set_index_register(){
        let mut emulator = Chip8::new();

        emulator.execute_command(Chip8Commands::SetIndexRegister(0xFFF));

        assert_eq!(emulator.index_regiser, 0xFFF);
    }

    #[test]
    fn test_set_register(){
        let mut emulator = Chip8::new();

        emulator.execute_command(Chip8Commands::SetRegister(2, 69));

        assert_eq!(emulator.registers[2], 69);
    }

    #[test]
    fn test_draw_command() {
        let mut emulator = Chip8::new();
        emulator.memory[0x200] = 0xFF;
        emulator.memory[0x201] = 0xFF;
        emulator.memory[0x202] = 0xFF;
        emulator.memory[0x203] = 0xFF;
        emulator.index_regiser = 0x200;
        emulator.registers[0] = 3;
        emulator.registers[1] = 2;

        let command = Chip8Commands::Draw(0, 1, 4);

        emulator.execute_command(command);
        
        for x in  0..64{
            for y in 0..32 {
                if (3..11).contains(&x) && (2..6).contains(&y){
                    assert!(emulator.display[x][y], "pixel {}, {} not set correctly", x, y);
                } else {
                    assert!(!emulator.display[x][y], "pixel {}, {} not set correctly", x, y);
                }
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
        emulator.registers[0] = 3;
        emulator.registers[1] = 2;

        let command = Chip8Commands::Draw(0, 1, 4);

        emulator.execute_command(command);
        
        for x in  0..64{
            for y in 0..32 {
                if (3..11).contains(&x) && (2..6).contains(&y){
                    if x == 3 && y == 2 {
                        assert!(!emulator.display[x][y], "pixel {}, {} not set correctly", x, y);
                    } else {
                        assert!(emulator.display[x][y], "pixel {}, {} not set correctly", x, y);
                    }
                } else {
                    assert!(!emulator.display[x][y], "pixel {}, {} not set correctly", x, y);
                }
            }
        }
        assert_eq!(emulator.registers[0xF], 1)
    }

    #[test]
    fn test_sprite_position_does_wrap(){
        let mut emulator = Chip8::new();
        emulator.memory[0x200] = 0xFF;
        emulator.memory[0x201] = 0xFF;
        emulator.memory[0x202] = 0xFF;
        emulator.memory[0x203] = 0xFF;
        emulator.index_regiser = 0x200;
        emulator.registers[0] = 66;
        emulator.registers[1] = 33;

        let command = Chip8Commands::Draw(0, 1, 4);

        emulator.execute_command(command);
        
        for x in 0..64 {
            for y in 0..32 {
                if (2..10).contains(&x) && (1..5).contains(&y) {
                    assert!(emulator.display[x][y], "pixel {}, {} not set correctly", x, y);
                } else {
                    assert!(!emulator.display[x][y], "pixel {}, {} not set correctly", x, y);
                }
            }
        }
    }

    #[test]
    fn test_draw_sprite_does_not_wrap(){
        let mut emulator = Chip8::new();
        emulator.memory[0x200] = 0xFF;
        emulator.memory[0x201] = 0xFF;
        emulator.memory[0x202] = 0xFF;
        emulator.memory[0x203] = 0xFF;
        emulator.index_regiser = 0x200;
        emulator.registers[0] = 62;
        emulator.registers[1] = 30;

        let command = Chip8Commands::Draw(0, 1, 4);

        emulator.execute_command(command);
        
        for x in 0..64 {
            for y in 0..32 {
                if (62..64).contains(&x) && (30..32).contains(&y) {
                    assert!(emulator.display[x][y], "pixel {}, {} not set correctly", x, y);
                } else {
                    assert!(!emulator.display[x][y], "pixel {}, {} not set correctly", x, y);
                }
            }
        }
    }

    #[test]
    fn test_draw_bit_order() {
        let mut emulator = Chip8::new();
        emulator.memory[0x200] = 0b11110000;
        emulator.index_regiser = 0x200;
        emulator.registers[0] = 0;
        emulator.registers[1] = 0;

        let command = Chip8Commands::Draw(0, 1, 1);

        emulator.execute_command(command);
        
        for x in 0..64 {
            for y in 0..32 {
                if y == 0 && (0..4).contains(&x){
                    assert!(emulator.display[x][y], "pixel {}, {} not set correctly", x, y);
                } else {
                    assert!(!emulator.display[x][y], "pixel {}, {} not set correctly", x, y);
                }
            }
        }
    }

    #[test]
    fn test_return() {
        let mut emulator = Chip8::new();
        emulator.stack = vec![0x208];
        emulator.program_counter = 0x500;

        emulator.execute_command(Chip8Commands::Return);

        assert_eq!(emulator.program_counter, 0x208);
    }

    #[test]
    fn test_skip_equal_to_register(){
        let mut emulator = Chip8::new();
        emulator.registers[0] = 6;
        emulator.program_counter = 0x200;
        let command = Chip8Commands::SkipEqualX(0, 0x6);

        emulator.execute_command(command);

        assert_eq!(emulator.program_counter, 0x202)
    }

    #[test]
    fn test_skip_equal_to_register_not_equal(){
        let mut emulator = Chip8::new();
        emulator.registers[0] = 7;
        emulator.program_counter = 0x200;
        let command = Chip8Commands::SkipEqualX(0, 0x6);

        emulator.execute_command(command);

        assert_eq!(emulator.program_counter, 0x200)
    }

    #[test]
    fn test_not_skip_equal_to_register(){
        let mut emulator = Chip8::new();
        emulator.registers[0] = 6;
        emulator.program_counter = 0x200;
        let command = Chip8Commands::SkipNotEqualX(0, 0x7);

        emulator.execute_command(command);

        assert_eq!(emulator.program_counter, 0x202)
    }

    #[test]
    fn test_skip_not_equal_to_register_equal(){
        let mut emulator = Chip8::new();
        emulator.registers[0] = 7;
        emulator.program_counter = 0x200;
        let command = Chip8Commands::SkipNotEqualX(0, 0x7);

        emulator.execute_command(command);

        assert_eq!(emulator.program_counter, 0x200)
    }

    #[test]
    fn test_not_equal_to_registers(){
        let mut emulator = Chip8::new();
        emulator.registers[0] = 6;
        emulator.registers[1] = 6;
        emulator.program_counter = 0x200;
        let command = Chip8Commands::SkipEqualXY(0, 1);

        emulator.execute_command(command);

        assert_eq!(emulator.program_counter, 0x202)
    }

    #[test]
    fn test_skip_equal_to_registers_not_equal(){
        let mut emulator = Chip8::new();
        emulator.registers[0] = 7;
        emulator.registers[1] = 6;
        emulator.program_counter = 0x200;
        let command = Chip8Commands::SkipEqualXY(0, 1);

        emulator.execute_command(command);

        assert_eq!(emulator.program_counter, 0x200)
    }

    #[test]
    fn test_not_not_equal_to_registers(){
        let mut emulator = Chip8::new();
        emulator.registers[0] = 6;
        emulator.registers[1] = 6;
        emulator.program_counter = 0x200;
        let command = Chip8Commands::SkipNotEqualXY(0, 1);

        emulator.execute_command(command);

        assert_eq!(emulator.program_counter, 0x200)
    }

    #[test]
    fn test_skip_not_equal_to_registers_equal(){
        let mut emulator = Chip8::new();
        emulator.registers[0] = 7;
        emulator.registers[1] = 6;
        emulator.program_counter = 0x200;
        let command = Chip8Commands::SkipNotEqualXY(0, 1);

        emulator.execute_command(command);

        assert_eq!(emulator.program_counter, 0x202)
    }

    #[test]
    fn test_load() {
        let mut emulator = Chip8::new();
        let command = Chip8Commands::Load(3, 69);

        emulator.execute_command(command);

        assert_eq!(emulator.registers[3], 69)
    }

    #[test]
    fn test_load_register_to_register(){
        let mut emulator = Chip8::new();
        emulator.registers[0] = 0;
        emulator.registers[5] = 70;
        let command = Chip8Commands::Load(0, 5);
        emulator.execute_command(command);

        assert_eq!(emulator.registers[0], 70);
    }

    #[test]
    fn test_bitwise_or() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 0b10101110;
        emulator.registers[5] = 0b01010000;

        emulator.execute_command(Chip8Commands::OR(0, 5));

        assert_eq!(emulator.registers[0], 0xFE)
    }    

    #[test]
    fn test_bitwise_and() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 0b10101111;
        emulator.registers[5] = 0b01010001;

        emulator.execute_command(Chip8Commands::AND(0, 5));
        
        assert_eq!(emulator.registers[0], 1)
    }

    #[test]
    fn test_bitwise_xor() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 0b10101111;
        emulator.registers[5] = 0b01010001;

        emulator.execute_command(Chip8Commands::XOR(0, 5));

        assert_eq!(emulator.registers[0], 0b11111110);
    }

    #[test]
    fn test_add_registers() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 6;
        emulator.registers[5] = 5;

        emulator.execute_command(Chip8Commands::ADD(0, 5));

        assert_eq!(emulator.registers[0], 11);
    }

    #[test]
    fn test_add_registers_overflow() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 255;
        emulator.registers[5] = 5;

        emulator.execute_command(Chip8Commands::ADD(0, 5));

        assert_eq!(emulator.registers[0], 4);
        assert_eq!(emulator.registers[0xF], 1);
    }

    #[test]
    fn test_sub_registers() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 6;
        emulator.registers[5] = 5;

        emulator.execute_command(Chip8Commands::SUB(0, 5));

        assert_eq!(emulator.registers[0], 1);
        assert_eq!(emulator.registers[0xF], 1);
    }

    #[test]
    fn test_sub_registers_borrow() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 5;
        emulator.registers[5] = 6;

        emulator.execute_command(Chip8Commands::SUB(0, 5));

        assert_eq!(emulator.registers[0], 255);
        assert_eq!(emulator.registers[0xF], 0);
    }

    #[test]
    fn test_shift_right_bit_1() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 0xFE;
        emulator.execute_command(Chip8Commands::ShiftRight(0, 5));

        assert_eq!(emulator.registers[0], 0x7F);
        assert_eq!(emulator.registers[0xF], 0);
    }

    #[test]
    fn test_shift_right_bit_0() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 0xFF;
        emulator.registers[5] = 5;

        emulator.execute_command(Chip8Commands::ShiftRight(0, 5));

        assert_eq!(emulator.registers[0], 0x7F);
        assert_eq!(emulator.registers[0xF], 1);
    }

    #[test]
    fn test_shift_right_bit_1_vy_used() {
        let mut emulator = Chip8::new();
        emulator.use_old_bit_shift = true;
        emulator.registers[5] = 0xFE;

        emulator.execute_command(Chip8Commands::ShiftRight(0, 5));

        assert_eq!(emulator.registers[0], 0x7F);
        assert_eq!(emulator.registers[0xF], 0);
    }

    #[test]
    fn test_shift_right_bit_0_vy_used() {
        let mut emulator = Chip8::new();
        emulator.use_old_bit_shift = true;
        emulator.registers[5] = 0xFF;

        emulator.execute_command(Chip8Commands::ShiftRight(0, 5));

        assert_eq!(emulator.registers[0], 0x7F);
        assert_eq!(emulator.registers[0xF], 1);
    }
}
