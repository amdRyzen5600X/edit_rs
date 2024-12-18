use std::{error::Error, result, time::Duration, u16};

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Position, Rect},
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::{errors::FileNameError, file_handler::FileHandler};

pub type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug, Default, Clone)]
pub struct App {
    pub mode: Mode,
    pub file_handler: FileHandler,
    pub scroll_bar_state: usize,
    pub cursor_position: Position,
    footer_text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Inesrt,
    Command,
    Quit,
}

impl Mode {
    fn str(&self) -> String {
        match self {
            &Self::Normal => "Normal".to_string(),
            &Self::Inesrt => "Inesrt".to_string(),
            &Self::Command => "Command".to_string(),
            &Self::Quit => "Quit".to_string(),
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Normal
    }
}

impl App {
    pub fn new(file_handler: FileHandler) -> Self {
        Self {
            mode: Mode::default(),
            file_handler,
            scroll_bar_state: usize::default(),
            cursor_position: Position::new(1, 2),
            footer_text: "placeholder".to_string(),
        }
    }
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.is_running() {
            terminal.draw(|frame| self.clone().draw(frame))?;
            self.handle_cursor(&mut terminal)?;
            self.handle_events()?;
            let _ = terminal.show_cursor();
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.mode != Mode::Quit
    }

    fn draw(self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f64(1.0 / 60.0);
        if !event::poll(timeout)? {
            return Ok(());
        }
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_key_press(key),
            _ => {}
        }
        Ok(())
    }

    fn handle_key_press(&mut self, key: KeyEvent) {
        match key.code {
            event::KeyCode::Char('q') if self.mode == Mode::Command => self.mode = Mode::Quit,
            event::KeyCode::Char('w') if self.mode == Mode::Command => self.save_file(),
            event::KeyCode::Char('i') if self.mode == Mode::Normal => self.mode = Mode::Inesrt,
            event::KeyCode::Char(':') if self.mode == Mode::Normal => self.mode = Mode::Command,
            event::KeyCode::Esc if self.mode == Mode::Inesrt || self.mode == Mode::Command => {
                self.mode = Mode::Normal
            }
            event::KeyCode::Char('h') if self.mode == Mode::Normal => {
                self.handle_cursor_movement('h')
            }
            event::KeyCode::Char('j') if self.mode == Mode::Normal => {
                self.handle_cursor_movement('j')
            }
            event::KeyCode::Char('k') if self.mode == Mode::Normal => {
                self.handle_cursor_movement('k')
            }
            event::KeyCode::Char('l') if self.mode == Mode::Normal => {
                self.handle_cursor_movement('l')
            }
            event::KeyCode::Char(ch) if self.mode == Mode::Inesrt => self.insert_char(ch),

            _ => {}
        }
    }

    fn insert_char(&self, ch: char) {
        todo!("concider how to get idx for Rope.insert_char(idx: usize, ch: char)")
    }

    fn handle_cursor_movement(&mut self, movement_char: char) {
        match movement_char {
            'h' => self.handle_h(),
            'j' => self.handle_j(),
            'k' => self.handle_k(),
            'l' => self.handle_l(),
            _ => {}
        }
    }

    fn handle_h(&mut self) {
        if self.cursor_position.x == 1 {
            return;
        }
        self.cursor_position.x -= 1;
    }

    fn handle_j(&mut self) {
        let max = self.file_handler.file_contents.len_lines();
        if usize::from(self.cursor_position.y) == max {
            return;
        }
        self.cursor_position.y += 1;
    }

    fn handle_k(&mut self) {
        if self.cursor_position.y == 2 {
            return;
        }
        self.cursor_position.y -= 1;
    }

    fn handle_l(&mut self) {
        let line = self
            .file_handler
            .file_contents
            .get_line((self.cursor_position.y - 2).into())
            .expect(&format!(
                "unable to get line at index {}",
                self.cursor_position.y - 2
            ));
        if usize::from(self.cursor_position.x) == line.len_chars() - 1 {
            return;
        }
        self.cursor_position.x += 1;
    }

    fn handle_cursor(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        terminal.show_cursor()?;
        terminal.set_cursor_position(self.cursor_position)?;
        Ok(())
    }
}

impl Widget for App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Max(3),
        ]);
        let [header, contents, footer] = vertical.areas(area);
        Block::new()
            .style(Style::new().bg(Color::Black))
            .render(area, buf);
        Paragraph::new(Text::raw(
            &self.file_handler.file_name.clone().unwrap_or_default(),
        ))
        .white()
        .render(header, buf);
        Paragraph::new(Text::raw(&self.file_handler.file_contents))
            .white()
            .block(Block::bordered())
            .scroll((self.scroll_bar_state as u16, 0))
            .render(contents, buf);
        self.render_footer(footer, buf);
    }
}

impl App {
    fn save_file(&mut self) {
        match self.file_handler.save_file() {
            Ok(_) => self.footer_text = "file saved".to_string(),
            Err(err) => {
                if format!("{}", err) == format!("{}", FileNameError {}) {
                    self.footer_text = "provide a file name".to_string();
                }
            }
        }
    }

    fn render_footer(&self, area: Rect, buff: &mut Buffer) {
        let horiz = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);
        let [left, right] = horiz.areas(area);
        Paragraph::new(self.mode.str())
            .white()
            .block(Block::bordered())
            .render(left, buff);
        Paragraph::new(self.footer_text.as_str())
            .white()
            .block(Block::bordered())
            .render(right, buff);
    }
}
