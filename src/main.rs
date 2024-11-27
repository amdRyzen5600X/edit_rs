use std::{
    env::args,
    fs::File,
    io::{self, Read},
};

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::{self, Block, Paragraph},
    DefaultTerminal,
};

fn main() -> io::Result<()> {
    let mut args = args();
    let _process_name = args.next();
    let file_name = args.next();
    let mut file = open_or_create_file(&file_name);
    let mut file_contents = Vec::new();
    file.read_to_end(&mut file_contents);
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_res = run(
        terminal,
        &file_name,
        &String::from_utf8(file_contents.to_vec()).unwrap(),
    );
    ratatui::restore();
    app_res
}

fn run(
    mut terminal: DefaultTerminal,
    fine_name: &Option<String>,
    file_contents: &String,
) -> io::Result<()> {
    loop {
        terminal.draw(move |frame| {
            let text_block = Paragraph::new(file_contents.to_string())
                .block(Block::bordered().title(fine_name.clone().unwrap_or("new_file".to_string())))
                .white();
            frame.render_widget(text_block, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}

fn open_or_create_file(file_name: &Option<String>) -> File {
    match file_name {
        Some(file_name) => match File::open(file_name) {
            Ok(file) => return file,
            Err(_) => File::create_new(file_name).unwrap(),
        },
        None => File::create_new("default_file.txt").unwrap(),
    }
}
