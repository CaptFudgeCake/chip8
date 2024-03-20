use std::io::{self, stdout, Stdout, Write};

use crossterm::{cursor, style::{self, Stylize}, terminal, ExecutableCommand, QueueableCommand};

use super::Display;


pub(crate) struct CrossTermDisplay {
    stdout: Stdout
}

impl CrossTermDisplay {
    pub(crate) fn new() -> CrossTermDisplay {
        let mut stdout = stdout();
        let _ = stdout.execute(cursor::Hide);
        let _ = stdout.execute(terminal::Clear(terminal::ClearType::All));
        CrossTermDisplay {
            stdout
        }
    }
}

impl Display for CrossTermDisplay {
    fn draw_display(&mut self, display_data: &[[bool; 32]; 64]) -> Result<(), io::Error> {
        for y in 0..32 {
            for x in 0..64 {
                let x0 = x * 2;
                let x1 = x0 + 1;
                if display_data[x][y] {
                    self.stdout
                        .queue(cursor::MoveTo(x0.try_into().unwrap(), y.try_into().unwrap()))?
                        .queue(style::PrintStyledContent("█".white()))?
                        .queue(cursor::MoveTo(x1.try_into().unwrap(), y.try_into().unwrap()))?
                        .queue(style::PrintStyledContent("█".white()))?;
                } else {
                    self.stdout
                        .queue(cursor::MoveTo(x0.try_into().unwrap(), y.try_into().unwrap()))?
                        .queue(style::PrintStyledContent("█".hidden()))?
                        .queue(cursor::MoveTo(x1.try_into().unwrap(), y.try_into().unwrap()))?
                        .queue(style::PrintStyledContent("█".hidden()))?;
                }
            }
        }
        self.stdout.flush()
    }
}