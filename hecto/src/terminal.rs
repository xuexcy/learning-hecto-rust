use core::fmt::Display;
use std::io::{stdout, Write};

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{queue, Command};

use crate::errors::IoError;

#[derive(Copy, Clone)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}
#[derive(Copy, Clone, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

pub struct Terminal {}

impl Terminal {
    pub fn terminate() -> IoError {
        Self::execute()?;
        disable_raw_mode()?;
        return Ok(());
    }
    pub fn initialize() -> IoError {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::execute()?;
        return Ok(());
    }
    pub fn clear_screen() -> IoError {
        Self::queue_command(Clear(ClearType::All))?;
        return Ok(());
    }
    pub fn move_caret_to(pos: Position) -> IoError {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(pos.col as u16, pos.row as u16))?;
        return Ok(());
    }
    pub fn clear_line() -> IoError {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        return Ok(());
    }
    pub fn hide_caret() -> IoError {
        Self::queue_command(Hide)?;
        return Ok(());
    }
    pub fn show_caret() -> IoError {
        Self::queue_command(Show)?;
        return Ok(());
    }
    pub fn print<T: Display>(string: T) -> IoError {
        Self::queue_command(Print(string))?;
        return Ok(());
    }
    pub fn size() -> Result<Size, std::io::Error> {
        let (width_u16, height_u16) = size()?;
        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;
        #[allow(clippy::as_conversions)]
        let width = width_u16 as usize;
        return Ok(Size { height, width });
    }
    pub fn execute() -> IoError {
        stdout().flush()?;
        return Ok(());
    }

    fn queue_command<T: Command>(command: T) -> IoError {
        queue!(stdout(), command)?;
        return Ok(());
    }
}
