mod chip8_commands;
mod display;
mod commands;

use std::{
    fs::File,
    io::Read,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread, time,
};

use chip8_commands::Chip8Commands;
use display::{display::CrossTermDisplay, Display};
use crate::commands::command_parser::parse_command;

struct Chip8 {
    memory: [u8; 4096],
    display_data: [[bool; 32]; 64],
    program_counter: u16,
    index_register: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    registers: [u8; 16],
    display_changed: bool,
    use_old_bit_shift: bool,
    display: Box<dyn Display>,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let display = Box::new(CrossTermDisplay::new());

        let mut new_chip8 = Chip8 {
            memory: [0; 4096],
            display_data: [[false; 32]; 64],
            program_counter: 0x200,
            index_register: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; 16],
            display_changed: false,
            use_old_bit_shift: false,
            display,
        };

        new_chip8.set_defaults();
        new_chip8
    }

    fn set_defaults(&mut self) {
        self.set_fonts();
    }

    pub fn load_program(&mut self, program: &[u8]) {
        let mut address = 0x200;
        if (address + program.len()) > 4096 {
            panic!("Program too long to fit in ram, solution not implemented")
        }

        for byte in program {
            self.memory[address] = *byte;
            address += 1;
        }
    }

    pub fn start(&mut self) {
        let target_ft = time::Duration::from_secs(1) / 700;
        // let start = time::Instant::now();
        let close_signal = Arc::new(AtomicBool::new(false));
        let close_signal_in_closure = close_signal.clone();
        ctrlc::set_handler(move || {
            close_signal_in_closure.store(true, Ordering::SeqCst);
        })
        .expect("Test");
        while !close_signal.load(Ordering::SeqCst) {
            let now = time::Instant::now();
            let command =
                &self.memory[(self.program_counter as usize)..(self.program_counter as usize + 2)];
            self.program_counter += 2;
            let decoded_command = parse_command(command);
            decoded_command.execute(self);
            if self.display_changed {
                self.display
                    .draw_display(&self.display_data)
                    .expect("Failed to draw display to console");
                self.display_changed = false;
            }
            if let Some(i) = target_ft.checked_sub(now.elapsed()) {
                thread::sleep(i);
            }
        }
        self.display.close_display();
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
}

fn main() {
    let mut program: Vec<u8> = Vec::new();
    let mut file = File::open("roms/4-flags.ch8").unwrap();
    file.read_to_end(&mut program)
        .expect("Failed to read program");
    let mut emulator = Chip8::new();
    emulator.load_program(&program);
    emulator.start();
}

#[cfg(test)]
mod test {
    use self::chip8_commands::Chip8Commands;
    use super::*;

    #[test]
    fn test_clear_screen() {
        let mut emulator = Chip8::new();
        for x in 0..64 {
            for y in 0..32 {
                emulator.display_data[x][y] = true;
            }
        }

        emulator.execute_command(Chip8Commands::ClearScreen);

        for row in emulator.display_data {
            for pixel in row {
                assert!(!pixel)
            }
        }
    }

    #[test]
    fn test_jump() {
        let mut emulator = Chip8::new();

        emulator.execute_command(Chip8Commands::Jump(0x22A));

        assert_eq!(emulator.program_counter, 0x22a);
    }

    #[test]
    fn test_add_to_register() {
        let mut emulator = Chip8::new();

        emulator.execute_command(Chip8Commands::AddValueToRegister(2, 6));
        emulator.execute_command(Chip8Commands::AddValueToRegister(2, 9));

        assert_eq!(emulator.registers[2], 15);
    }

    #[test]
    fn test_add_to_register_overflow_shouldnt_fail() {
        let mut emulator = Chip8::new();

        emulator.execute_command(Chip8Commands::AddValueToRegister(2, 129));
        emulator.execute_command(Chip8Commands::AddValueToRegister(2, 128));

        assert_eq!(emulator.registers[2], 1);
    }

    #[test]
    fn test_set_index_register() {
        let mut emulator = Chip8::new();

        emulator.execute_command(Chip8Commands::SetIndexRegister(0xFFF));

        assert_eq!(emulator.index_register, 0xFFF);
    }

    #[test]
    fn test_set_register() {
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
        emulator.index_register = 0x200;
        emulator.registers[0] = 3;
        emulator.registers[1] = 2;

        let command = Chip8Commands::Draw(0, 1, 4);

        emulator.execute_command(command);

        for x in 0..64 {
            for y in 0..32 {
                if (3..11).contains(&x) && (2..6).contains(&y) {
                    assert!(
                        emulator.display_data[x][y],
                        "pixel {}, {} not set correctly",
                        x, y
                    );
                } else {
                    assert!(
                        !emulator.display_data[x][y],
                        "pixel {}, {} not set correctly",
                        x, y
                    );
                }
            }
        }
    }

    #[test]
    fn test_draw_ovewrite() {
        let mut emulator = Chip8::new();
        emulator.memory[0x200] = 0xFF;
        emulator.memory[0x201] = 0xFF;
        emulator.memory[0x202] = 0xFF;
        emulator.memory[0x203] = 0xFF;
        emulator.index_register = 0x200;
        emulator.display_data[3][2] = true;
        emulator.registers[0] = 3;
        emulator.registers[1] = 2;

        let command = Chip8Commands::Draw(0, 1, 4);

        emulator.execute_command(command);

        for x in 0..64 {
            for y in 0..32 {
                if (3..11).contains(&x) && (2..6).contains(&y) {
                    if x == 3 && y == 2 {
                        assert!(
                            !emulator.display_data[x][y],
                            "pixel {}, {} not set correctly",
                            x, y
                        );
                    } else {
                        assert!(
                            emulator.display_data[x][y],
                            "pixel {}, {} not set correctly",
                            x, y
                        );
                    }
                } else {
                    assert!(
                        !emulator.display_data[x][y],
                        "pixel {}, {} not set correctly",
                        x, y
                    );
                }
            }
        }
        assert_eq!(emulator.registers[0xF], 1);
        assert!(emulator.display_changed);
    }

    #[test]
    fn test_sprite_position_does_wrap() {
        let mut emulator = Chip8::new();
        emulator.memory[0x200] = 0xFF;
        emulator.memory[0x201] = 0xFF;
        emulator.memory[0x202] = 0xFF;
        emulator.memory[0x203] = 0xFF;
        emulator.index_register = 0x200;
        emulator.registers[0] = 66;
        emulator.registers[1] = 33;

        let command = Chip8Commands::Draw(0, 1, 4);

        emulator.execute_command(command);

        for x in 0..64 {
            for y in 0..32 {
                if (2..10).contains(&x) && (1..5).contains(&y) {
                    assert!(
                        emulator.display_data[x][y],
                        "pixel {}, {} not set correctly",
                        x, y
                    );
                } else {
                    assert!(
                        !emulator.display_data[x][y],
                        "pixel {}, {} not set correctly",
                        x, y
                    );
                }
            }
        }
        assert!(emulator.display_changed);
    }

    #[test]
    fn test_draw_sprite_does_not_wrap() {
        let mut emulator = Chip8::new();
        emulator.memory[0x200] = 0xFF;
        emulator.memory[0x201] = 0xFF;
        emulator.memory[0x202] = 0xFF;
        emulator.memory[0x203] = 0xFF;
        emulator.index_register = 0x200;
        emulator.registers[0] = 62;
        emulator.registers[1] = 30;

        let command = Chip8Commands::Draw(0, 1, 4);

        emulator.execute_command(command);

        for x in 0..64 {
            for y in 0..32 {
                if (62..64).contains(&x) && (30..32).contains(&y) {
                    assert!(
                        emulator.display_data[x][y],
                        "pixel {}, {} not set correctly",
                        x, y
                    );
                } else {
                    assert!(
                        !emulator.display_data[x][y],
                        "pixel {}, {} not set correctly",
                        x, y
                    );
                }
            }
        }
        assert!(emulator.display_changed);
    }

    #[test]
    fn test_draw_bit_order() {
        let mut emulator = Chip8::new();
        emulator.memory[0x200] = 0b11110000;
        emulator.index_register = 0x200;
        emulator.registers[0] = 0;
        emulator.registers[1] = 0;

        let command = Chip8Commands::Draw(0, 1, 1);

        emulator.execute_command(command);

        for x in 0..64 {
            for y in 0..32 {
                if y == 0 && (0..4).contains(&x) {
                    assert!(
                        emulator.display_data[x][y],
                        "pixel {}, {} not set correctly",
                        x, y
                    );
                } else {
                    assert!(
                        !emulator.display_data[x][y],
                        "pixel {}, {} not set correctly",
                        x, y
                    );
                }
            }
        }
        assert!(emulator.display_changed);
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
    fn test_skip_equal_to_register() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 6;
        emulator.program_counter = 0x200;
        let command = Chip8Commands::SkipEqualX(0, 0x6);

        emulator.execute_command(command);

        assert_eq!(emulator.program_counter, 0x202)
    }

    #[test]
    fn test_skip_equal_to_register_not_equal() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 7;
        emulator.program_counter = 0x200;
        let command = Chip8Commands::SkipEqualX(0, 0x6);

        emulator.execute_command(command);

        assert_eq!(emulator.program_counter, 0x200)
    }

    #[test]
    fn test_not_skip_equal_to_register() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 6;
        emulator.program_counter = 0x200;
        let command = Chip8Commands::SkipNotEqualX(0, 0x7);

        emulator.execute_command(command);

        assert_eq!(emulator.program_counter, 0x202)
    }

    #[test]
    fn test_skip_not_equal_to_register_equal() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 7;
        emulator.program_counter = 0x200;
        let command = Chip8Commands::SkipNotEqualX(0, 0x7);

        emulator.execute_command(command);

        assert_eq!(emulator.program_counter, 0x200)
    }

    #[test]
    fn test_not_equal_to_registers() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 6;
        emulator.registers[1] = 6;
        emulator.program_counter = 0x200;
        let command = Chip8Commands::SkipEqualXY(0, 1);

        emulator.execute_command(command);

        assert_eq!(emulator.program_counter, 0x202)
    }

    #[test]
    fn test_skip_equal_to_registers_not_equal() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 7;
        emulator.registers[1] = 6;
        emulator.program_counter = 0x200;
        let command = Chip8Commands::SkipEqualXY(0, 1);

        emulator.execute_command(command);

        assert_eq!(emulator.program_counter, 0x200)
    }

    #[test]
    fn test_not_not_equal_to_registers() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 6;
        emulator.registers[1] = 6;
        emulator.program_counter = 0x200;
        let command = Chip8Commands::SkipNotEqualXY(0, 1);

        emulator.execute_command(command);

        assert_eq!(emulator.program_counter, 0x200)
    }

    #[test]
    fn test_skip_not_equal_to_registers_equal() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 7;
        emulator.registers[1] = 6;
        emulator.program_counter = 0x200;
        let command = Chip8Commands::SkipNotEqualXY(0, 1);

        emulator.execute_command(command);

        assert_eq!(emulator.program_counter, 0x202)
    }

    #[test]
    fn test_load_register_to_register() {
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
    fn test_sub_reverse_registers() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 5;
        emulator.registers[5] = 6;

        emulator.execute_command(Chip8Commands::SUBN(0, 5));

        assert_eq!(emulator.registers[0], 1);
        assert_eq!(emulator.registers[0xF], 1);
    }

    #[test]
    fn test_sub_reverse_registers_borrow() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 6;
        emulator.registers[5] = 5;

        emulator.execute_command(Chip8Commands::SUBN(0, 5));

        assert_eq!(emulator.registers[0], 255);
        assert_eq!(emulator.registers[0xF], 0);
    }

    #[test]
    fn test_shift_right_bit_0() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 0xFE;
        emulator.execute_command(Chip8Commands::ShiftRight(0, 5));

        assert_eq!(emulator.registers[0], 0x7F);
        assert_eq!(emulator.registers[0xF], 0);
    }

    #[test]
    fn test_shift_right_bit_1() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 0xFF;
        emulator.execute_command(Chip8Commands::ShiftRight(0, 5));

        assert_eq!(emulator.registers[0], 0x7F);
        assert_eq!(emulator.registers[0xF], 1);
    }

    #[test]
    fn test_shift_right_bit_0_vy_used() {
        let mut emulator = Chip8::new();
        emulator.use_old_bit_shift = true;
        emulator.registers[5] = 0xFE;

        emulator.execute_command(Chip8Commands::ShiftRight(0, 5));

        assert_eq!(emulator.registers[0], 0x7F);
        assert_eq!(emulator.registers[0xF], 0);
    }

    #[test]
    fn test_shift_right_bit_1_vy_used() {
        let mut emulator = Chip8::new();
        emulator.use_old_bit_shift = true;
        emulator.registers[5] = 0xFF;

        emulator.execute_command(Chip8Commands::ShiftRight(0, 5));

        assert_eq!(emulator.registers[0], 0x7F);
        assert_eq!(emulator.registers[0xF], 1);
    }

    #[test]
    fn test_shift_left_bit_1() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 0xFF;
        emulator.execute_command(Chip8Commands::ShiftLeft(0, 5));

        assert_eq!(emulator.registers[0], 0xFE);
        assert_eq!(emulator.registers[0xF], 1);
    }

    #[test]
    fn test_shift_left_bit_0() {
        let mut emulator = Chip8::new();
        emulator.registers[0] = 0x7F;

        emulator.execute_command(Chip8Commands::ShiftLeft(0, 5));

        assert_eq!(emulator.registers[0], 0xFE);
        assert_eq!(emulator.registers[0xF], 0);
    }

    #[test]
    fn test_shift_left_bit_1_vy_used() {
        let mut emulator = Chip8::new();
        emulator.use_old_bit_shift = true;
        emulator.registers[5] = 0xFF;

        emulator.execute_command(Chip8Commands::ShiftLeft(0, 5));

        assert_eq!(emulator.registers[0], 0xFE);
        assert_eq!(emulator.registers[0xF], 1);
    }

    #[test]
    fn test_shift_left_bit_0_vy_used() {
        let mut emulator = Chip8::new();
        emulator.use_old_bit_shift = true;
        emulator.registers[5] = 0x7F;

        emulator.execute_command(Chip8Commands::ShiftLeft(0, 5));

        assert_eq!(emulator.registers[0], 0xFE);
        assert_eq!(emulator.registers[0xF], 0);
    }

    #[test]
    fn test_binary_coded_decimal_htu() {
        let mut emulator = Chip8::new();
        emulator.index_register = 0x200;
        emulator.registers[0] = 235;

        emulator.execute_command(Chip8Commands::BinaryCodedDecimal(0));

        assert_eq!(emulator.memory[0x200], 2);
        assert_eq!(emulator.memory[0x201], 3);
        assert_eq!(emulator.memory[0x202], 5);
    }

    #[test]
    fn test_binary_coded_decimal_hu() {
        let mut emulator = Chip8::new();
        emulator.index_register = 0x200;
        emulator.registers[0] = 205;

        emulator.execute_command(Chip8Commands::BinaryCodedDecimal(0));

        assert_eq!(emulator.memory[0x200], 2);
        assert_eq!(emulator.memory[0x201], 0);
        assert_eq!(emulator.memory[0x202], 5);
    }

    #[test]
    fn test_binary_coded_decimal_u() {
        let mut emulator = Chip8::new();
        emulator.index_register = 0x200;
        emulator.registers[0] = 5;

        emulator.execute_command(Chip8Commands::BinaryCodedDecimal(0));

        assert_eq!(emulator.memory[0x200], 0);
        assert_eq!(emulator.memory[0x201], 0);
        assert_eq!(emulator.memory[0x202], 5);
    }

    #[test]
    fn test_binary_coded_decimal_h() {
        let mut emulator = Chip8::new();
        emulator.index_register = 0x200;
        emulator.registers[0] = 200;

        emulator.execute_command(Chip8Commands::BinaryCodedDecimal(0));

        assert_eq!(emulator.memory[0x200], 2);
        assert_eq!(emulator.memory[0x201], 0);
        assert_eq!(emulator.memory[0x202], 0);
    }

    #[test]
    fn test_binary_coded_decimal_t() {
        let mut emulator = Chip8::new();
        emulator.index_register = 0x200;
        emulator.registers[0] = 30;

        emulator.execute_command(Chip8Commands::BinaryCodedDecimal(0));

        assert_eq!(emulator.memory[0x200], 0);
        assert_eq!(emulator.memory[0x201], 3);
        assert_eq!(emulator.memory[0x202], 0);
    }

    #[test]
    fn test_store_registers_in_memory() {
        let mut emulator = Chip8::new();
        emulator.index_register = 0x200;
        emulator.registers[0] = 30;
        emulator.registers[1] = 12;
        emulator.registers[2] = 89;
        emulator.registers[3] = 23;
        emulator.registers[4] = 65;
        emulator.registers[5] = 34;
        emulator.registers[6] = 67;
        emulator.registers[7] = 88;

        emulator.execute_command(Chip8Commands::StoreRegisters(7));

        assert_eq!(emulator.memory[0x200], 30);
        assert_eq!(emulator.memory[0x201], 12);
        assert_eq!(emulator.memory[0x202], 89);
        assert_eq!(emulator.memory[0x203], 23);
        assert_eq!(emulator.memory[0x204], 65);
        assert_eq!(emulator.memory[0x205], 34);
        assert_eq!(emulator.memory[0x206], 67);
        assert_eq!(emulator.memory[0x207], 88);
    }

    #[test]
    fn test_command_read_into_registers() {
        let mut emulator = Chip8::new();
        emulator.index_register = 0x200;
        emulator.registers[0] = 0;
        emulator.registers[1] = 0;

        emulator.memory[0x200] = 20;
        emulator.memory[0x201] = 21;

        emulator.execute_command(Chip8Commands::ReadIntoRegisters(1));

        assert_eq!(emulator.registers[0], 20);
        assert_eq!(emulator.registers[1], 21);
    }

    #[test]
    fn test_add_to_index() {
        let mut emulator = Chip8::new();
        emulator.index_register = 0x200;
        emulator.registers[5] = 10;

        emulator.execute_command(Chip8Commands::AddToIndex(5));

        assert_eq!(emulator.index_register, 0x20A);
    }

    #[test]
    fn test_call_function() {
        let mut emulator = Chip8::new();
        emulator.program_counter = 0x200;

        emulator.execute_command(Chip8Commands::Call(0x543));

        assert_eq!(emulator.stack.len(), 1);
        assert_eq!(emulator.stack[0], 0x200);
        assert_eq!(emulator.program_counter, 0x543);
    }
}
