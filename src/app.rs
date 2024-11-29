use std::{error::Error, result, time::Duration};

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

pub type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct App {
    pub mode: Mode,
    pub contents: ropey::Rope,
    pub file_name: String,
    pub scroll_bar_state: usize,
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
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.is_running() {
            terminal.draw(|frame| self.clone().draw(frame))?;
            self.handle_events()?;
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
            event::KeyCode::Char('q') if self.mode == Mode::Normal => self.mode = Mode::Quit,
            event::KeyCode::Char('i') if self.mode == Mode::Normal => self.mode = Mode::Inesrt,
            event::KeyCode::Char(':') if self.mode == Mode::Normal => self.mode = Mode::Command,
            event::KeyCode::Esc if self.mode == Mode::Inesrt || self.mode == Mode::Command => {
                self.mode = Mode::Normal
            }
            _ => {}
        }
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
        Paragraph::new(Text::raw(&self.file_name))
            .white()
            .render(header, buf);
        Paragraph::new(Text::raw(&self.contents))
            .white()
            .block(Block::bordered())
            .render(contents, buf);
        self.render_footer(footer, buf);
    }
}

impl App {
    fn render_footer(&self, area: Rect, buff: &mut Buffer) {
        let horiz = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);
        let [left, right] = horiz.areas(area);
        Paragraph::new(self.mode.str())
            .white()
            .block(Block::bordered())
            .render(left, buff);
        Paragraph::new("placeholder for line number")
            .white()
            .block(Block::bordered())
            .render(right, buff);
    }
}
