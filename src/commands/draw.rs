use crate::commands::command::Command;
use crate::Chip8;

pub struct Draw {
    register_x: u8,
    register_y: u8,
    bytes: u8,
}

impl Draw {
    pub fn new(register_x: u8, register_y: u8, bytes: u8) -> Self {
        Self {
            register_x,
            register_y,
            bytes,
        }
    }
}

impl Command for Draw {
    fn execute(&self, emulator: &mut Chip8) {
        let x_start = (emulator.registers[self.register_x as usize] as usize) % 64;
        let y_start = (emulator.registers[self.register_y as usize] as usize) % 32;
        for byte_offset in 0..self.bytes {
            let byte = emulator.memory[emulator.index_register as usize + byte_offset as usize];
            for i in 0..8 {
                let bit = ((byte >> 7 - i) & 0b1) != 0;
                let x_pos = x_start + i;
                let y_pos = y_start + byte_offset as usize;
                if x_pos < 64 && y_pos < 32 {
                    if emulator.display_data[x_pos][y_pos] != bit {
                        emulator.registers[0xF] = 1;
                    }
                    emulator.display_data[x_pos][y_pos] ^= bit;
                }
            }
        }

        emulator.display_changed = true;
    }
}

mod test {
    use crate::Chip8;
    use crate::commands::command::Command;
    use crate::commands::draw::Draw;

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

        let command = Draw::new(0, 1, 4);

        command.execute(&mut emulator);

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

        let command = Draw::new(0, 1, 4);

        command.execute(&mut emulator);

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

        let command = Draw::new(0, 1, 4);

        command.execute(&mut emulator);

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

        let command = Draw::new(0, 1, 4);

        command.execute(&mut emulator);

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

        let command = Draw::new(0, 1, 1);

        command.execute(&mut emulator);

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

}