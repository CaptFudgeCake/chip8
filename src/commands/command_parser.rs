use crate::commands::add::Add;
use crate::commands::add_to_index::AddToIndex;
use crate::commands::add_value_to_register::AddValueToRegister;
use crate::commands::and::And;
use crate::commands::binary_coded_decimal::BinaryCodedDecimal;
use crate::commands::call::Call;
use crate::commands::command::Command;
use crate::commands::clear_screen::ClearScreen;
use crate::commands::draw::Draw;
use crate::commands::jump::Jump;
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

pub fn parse_command(command: &[u8]) -> Box<dyn Command>{
    let opcode = (command[0] & 0xF0) >> 4;
    match opcode {
        0 => match command {
            [0x00, 0xE0] => Box::new(ClearScreen{}),
            [0x00, 0xEE] => Box::new(Return{}),
            _ => panic!("0NNN command can't be run since it is dependant on specific hardware"),
        },
        3 | 4 | 6 | 7 => {
            let x = command[0] & 0xF;
            match opcode {
                3 => Box::new(SkipEqualX::new(x, command[1])),
                4 => Box::new(SkipNotEqualX::new(x.into(), command[1])),
                6 => Box::new(SetRegister::new(x.into(), command[1])),
                7 => Box::new(AddValueToRegister::new(x.into(), command[1])),
                _ => panic!("Instruction {:x?} not found", command),
            }
        }
        8 => {
            let x = command[0] & 0xF;
            let y = (command[1] >> 4) & 0xF;
            let identifier = command[1] & 0xF;
            match identifier {
                0x0 => Box::new(Load::new(x.into(), y.into())),
                0x1 => Box::new(Or::new(x.into(), y.into())),
                0x2 => Box::new(And::new(x.into(), y.into())),
                0x3 => Box::new(Xor::new(x.into(), y.into())),
                0x4 => Box::new(Add::new(x.into(), y.into())),
                0x5 => Box::new(Sub::new(x.into(), y.into())),
                0x6 => Box::new(ShiftRight::new(x.into(), y.into())),
                0x7 => Box::new(SubN::new(x.into(), y.into())),
                0xE => Box::new(ShiftLeft::new(x.into(), y.into())),
                _ => panic!("Instruction {:x?} not found", command),
            }
        }
        0xF => {
            let x = command[0] & 0xF;
            match command[1] {
                0x1E => Box::new(AddToIndex::new(x.into())),
                0x33 => Box::new(BinaryCodedDecimal::new(x.into())),
                0x55 => Box::new(StoreRegisters::new(x.into())),
                0x65 => Box::new(ReadIntoRegisters::new(x.into())),
                _ => panic!("Instruction {:x?} not found", command),
            }
        }
        1 | 2 | 0xA => {
            let address = ((command[0] as u16 & 0xF) << 8) | command[1] as u16;
            match opcode {
                1 => Box::new(Jump::new(address)),
                2 => Box::new(Call::new(address)),
                0xA => Box::new(SetIndexRegister::new(address)),
                _ => panic!("Instruction {:x?} not found", command),
            }
        }
        5 => {
            let x = command[0] & 0xF;
            let y = (command[1] >> 4) & 0xF;
            Box::new(SkipEqualXY::new(x.into(), y.into()))
        }
        9 => {
            let x = command[0] & 0xF;
            let y = (command[1] >> 4) & 0xF;
            Box::new(SkipNotEqualXY::new(x.into(), y.into()))
        }
        0xD => {
            let x = command[0] & 0xF;
            let y = (command[1] & 0xF0) >> 4;
            let bytes = command[1] & 0xF;
            Box::new(Draw::new(x.into(), y.into(), bytes))
        }
        _ => panic!("Instruction {:x?} not found", command),
    }
}