mod chip8_commands;
mod commands;
mod display;

use std::{
    fs::File,
    io::Read,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread, time,
};

use crate::commands::command_parser::parse_command;
use display::{display::CrossTermDisplay, Display};

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
    let mut file = File::open("roms/5-quirks.ch8").unwrap();
    file.read_to_end(&mut program)
        .expect("Failed to read program");
    let mut emulator = Chip8::new();
    emulator.load_program(&program);
    emulator.start();
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::commands::add::Add;
    use crate::commands::add_to_index::AddToIndex;
    use crate::commands::add_value_to_register::AddValueToRegister;
    use crate::commands::and::And;
    use crate::commands::binary_coded_decimal::BinaryCodedDecimal;
    use crate::commands::call::Call;
    use crate::commands::command::Command;
    use crate::commands::draw::Draw;
    use crate::commands::load::Load;
    use crate::commands::or::Or;
    use crate::commands::read_into_registers::ReadIntoRegisters;
    use crate::commands::return_command::Return;
    use crate::commands::set_index_register::SetIndexRegister;
    use crate::commands::set_register::SetRegister;
    use crate::commands::shift_left::ShiftLeft;
    use crate::commands::shift_right::ShiftRight;
    use crate::commands::skip_equal_x::SkipEqualX;
    use crate::commands::skip_equal_x_y::SkipEqualXY;
    use crate::commands::skip_not_equal_x::SkipNotEqualX;
    use crate::commands::skip_not_equal_xy::SkipNotEqualXY;
    use crate::commands::store_registers::StoreRegisters;
    use crate::commands::sub::Sub;
    use crate::commands::sub_n::SubN;
    use crate::commands::xor::Xor;
}
