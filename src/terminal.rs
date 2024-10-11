use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType,
    },
};
use std::io::{self, stdout, Write};
use clearscreen;
use crate::Position;

pub struct Terminal {
    // _stdout: io::Stdout,
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = self.disable_raw_model();
    }
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        enable_raw_mode().unwrap();
        Ok(Self { 
        })
    }
    pub fn disable_raw_model(&mut self) -> Result<(), io::Error> {
        disable_raw_mode()?;
        Ok(())
    }
    pub fn clear_screen() {
        clearscreen::clear().unwrap();
    }

    pub fn cursor_position(position: &Position) {
        let &Position { x, y } = position;
        // x = x.saturating_add(1);
        // y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        execute!(stdout(), MoveTo(x, y)).unwrap();
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn read_key() -> Result<KeyCode, std::io::Error> {
        loop {
            if let Event::Key(event) = read()? {
                if event.kind == KeyEventKind::Press {
                    return Ok(event.code);
                }
            }
        }
    }

    pub fn read_line(input: &mut String) -> io::Result<usize> {
        std::io::stdin().read_line(input)
    }

    pub fn write(s: &str) {
        std::io::stdout().write(s.as_bytes()).unwrap();
    }

    pub fn cursor_hide() {
        execute!(stdout(), Hide).unwrap();
    }

    pub fn cursor_show() {
        execute!(stdout(), Show).unwrap();
    }

    pub fn clear_current_line() {
        execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();
    }
}
