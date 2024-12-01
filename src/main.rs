use std::env::args;

use edit_rs::file_handler::FileHandler;

fn main() -> edit_rs::app::Result<()> {
    let file_name = args().nth(1);
    let file_handler = FileHandler::new(file_name);

    let app = edit_rs::app::App::new(file_handler);
    let terminal = ratatui::init();
    let app_result = app.run(terminal);
    ratatui::restore();
    app_result
}
