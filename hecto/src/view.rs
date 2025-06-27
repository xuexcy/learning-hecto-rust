use crate::errors::IoError;
use crate::terminal::{Size, Terminal};

const WELCOME_MESSAGE: &'static str = concat!(
    "Name: ",
    env!("CARGO_PKG_NAME"),
    " , Version: ",
    env!("CARGO_PKG_VERSION")
);
const WELCOME_MESSAGE_LEN: usize = WELCOME_MESSAGE.len();

pub struct View;

impl View {
    pub fn render() -> IoError {
        let Size { height, .. } = Terminal::size()?;
        Terminal::clear_line();
        Terminal::print("Hello, Wrold!\r\n")?;
        for current_row in 1..height {
            Terminal::clear_line()?;
            #[allow(clippy::integer_division)]
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row();
            }
            // 不是最后一行就换行
            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n");
            }
        }
        return Ok(());
    }
    fn draw_welcome_message() -> IoError {
        let width = Terminal::size()?.width as usize;
        #[allow(clippy::integer_division)]
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

