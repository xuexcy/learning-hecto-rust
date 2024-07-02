use core::cmp::min;

use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode::{self, Char},
    KeyEvent, KeyEventKind, KeyModifiers,
};

use crate::errors::IoError;
use crate::terminal::Position;
use crate::terminal::Size;
use crate::terminal::Terminal;

const WELCOME_MESSAGE: &'static str = concat!(
    "Name: ",
    env!("CARGO_PKG_NAME"),
    " , Version: ",
    env!("CARGO_PKG_VERSION")
);
const WELCOME_MESSAGE_LEN: usize = WELCOME_MESSAGE.len();

//  ------> x
// |
// |
// |
// |
// V
// y
#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> IoError {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
        }
        return Ok(());
    }
    fn move_point(&mut self, key_code: KeyCode) -> IoError {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size()?;
        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            _ => (),
        }
        self.location = Location { x, y };
        return Ok(());
    }
    fn evaluate_event(&mut self, event: &Event) -> IoError {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                Char('d') if KeyModifiers::CONTROL == *modifiers => {
                    self.should_quit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_point(*code)?;
                }
                _ => (),
            }
        }
        return Ok(());
    }
    fn refresh_screen(&self) -> IoError {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            self.draw_rows()?;
            Terminal::move_caret_to(Position {
                col: self.location.x,
                row: self.location.y,
            })?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;
        return Ok(());
    }
    fn draw_rows(&self) -> IoError {
        let height = Terminal::size()?.height;
        for current_row in 0..height {
            Terminal::clear_line()?;
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        return Ok(());
    }
    fn draw_welcome_message() -> IoError {
        let width = Terminal::size()?.width as usize;
        let padding = (width - WELCOME_MESSAGE_LEN) / 2;
        let space = " ".repeat(padding - 1);
        let mut welcome_message = format!("~{space}{WELCOME_MESSAGE}");
        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;
        return Ok(());
    }
    fn draw_empty_row() -> IoError {
        Terminal::print("~")?;
        return Ok(());
    }
}
