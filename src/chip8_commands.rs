#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) enum Chip8Commands {
    ClearScreen,                // 00E0
    Return,                     // 00EE
    Jump(u16),                  // 1NNN
    Call(u16),                  // 2NNN
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
    SUBN(u8, u8),               // 8XY7
    ShiftLeft(u8, u8),          // 8XYE
    SkipNotEqualXY(u8, u8),     // 9XY0
    SetIndexRegister(u16),      // ANNN
    Draw(u8, u8, u8),           // DXYN
    BinaryCodedDecimal(u8),     // FX33
    StoreRegisters(u8),         // FX55
    ReadIntoRegisters(u8),      // FX65
}

impl Chip8Commands {
    pub fn new(command: &[u8]) -> Chip8Commands {
        let opcode = (command[0] & 0xF0) >> 4;
        match opcode {
            0 => match command {
                [0x00, 0xE0] => Chip8Commands::ClearScreen,
                [0x00, 0xEE] => Chip8Commands::Return,
                _ => panic!("0NNN command can't be run since it is dependant on specific hardware"),
            },
            3 | 4 | 6 | 7 => {
                let x = command[0] & 0xF;
                match opcode {
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
                let identifier = command[1] & 0xF;
                match identifier {
                    0x0 => Chip8Commands::Load(x.into(), y.into()),
                    0x1 => Chip8Commands::OR(x.into(), y.into()),
                    0x2 => Chip8Commands::AND(x.into(), y.into()),
                    0x3 => Chip8Commands::XOR(x.into(), y.into()),
                    0x4 => Chip8Commands::ADD(x.into(), y.into()),
                    0x5 => Chip8Commands::SUB(x.into(), y.into()),
                    0x6 => Chip8Commands::ShiftRight(x.into(), y.into()),
                    0x7 => Chip8Commands::SUBN(x.into(), y.into()),
                    0xE => Chip8Commands::ShiftLeft(x.into(), y.into()),
                    _ => panic!("Instruction {:x?} not found", command),
                }
            }
            0xF => {
                let x = command[0] & 0xF;
                match command[1] {
                    0x33 => Chip8Commands::BinaryCodedDecimal(x.into()),
                    0x55 => Chip8Commands::StoreRegisters(x.into()),
                    0x65 => Chip8Commands::ReadIntoRegisters(x.into()),
                    _ => panic!("Instruction {:x?} not found", command),
                }
            }
            1 | 2 | 0xA => {
                let address = ((command[0] as u16 & 0xF) << 8) | command[1] as u16;
                match opcode {
                    1 => Chip8Commands::Jump(address),
                    2 => Chip8Commands::Call(address),
                    0xA => Chip8Commands::SetIndexRegister(address),
                    _ => panic!("Instruction {:x?} not found", command),
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_command_decode() {
        let commands: [[u8; 2]; 24] = [
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
            [0xF6, 0x55],
            [0x2A, 0x53],
            [0x83, 0x67],
            [0xF1, 0x65],
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
            Chip8Commands::Call(0xA53),
            Chip8Commands::SUBN(3, 6),
            Chip8Commands::ReadIntoRegisters(1),
        ];

        for (i, command) in commands.into_iter().enumerate() {
            let result = Chip8Commands::new(&command);
            let expected = &expected[i];

            assert_eq!(result, *expected)
        }
    }
}
