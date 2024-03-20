use std::io;

pub(crate) mod display;

pub(crate) trait Display {
    fn draw_display(&mut self, display_data: &[[bool; 32]; 64]) -> Result<(), io::Error> ;
}