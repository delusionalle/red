use std::{
    io,
};
use termion::event::Key;
use crate::document::Document;
use crate::row::Row;
use crate::terminal::Terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_pos: Position,
    document: Document,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(e) = self.refresh_screen() {
                die(e);
            }

            if self.should_quit {
                break;
            }

            if let Err(e) = self.process_keypress() {
                die(e);
            }
        }
    }

    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("failed to init terminal obj"),
            cursor_pos: Position::default(),
            document: Document::default(),
        }
    }

    fn refresh_screen(&self) -> Result<(), io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_pos(&Position::default());
        if self.should_quit {
            Terminal::clear();
            println!("goodbye!");
        } else {
            self.draw_rows();
            Terminal::cursor_pos(&self.cursor_pos);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up | Key::Down | Key::Right | Key::Left | Key::PageUp | Key::PageDown | Key::End | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_pos;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_sub(1);
                }
            }
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }

        self.cursor_pos = Position { x, y };
    }

    fn draw_welcome_msg(&self) {
        let mut welcome_msg = format!("red | v{}\r", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_msg.len();
        let pad = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(pad.saturating_sub(1));
        welcome_msg = format!("~{}{}", spaces, welcome_msg);
        welcome_msg.truncate(width);
        println!("{}\r", welcome_msg);
    }

    fn draw_row(&self, row: &Row) {
        let start = 0;
        let end = self.terminal.size().width as usize;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for row_index in 0..height - 1 {
            Terminal::clear_cur_line();
            if let Some(row) = self.document.row(row_index as usize) {
                self.draw_row(row);
            } else if self.document.is_empty() && row_index == height / 3 {
                self.draw_welcome_msg();
            } else {
                println!("~\r");
            }
        }
    }
}

fn die(e: io::Error) {
    Terminal::clear();
    panic!("{}", e)
}
