use std::fs::File;

use ropey::Rope;

fn main() -> edit_rs::app::Result<()> {
    let mut app = edit_rs::app::App {
        file_name: "sometext.txt".to_string(),
        contents: Rope::from_reader(File::open("sometext.txt").expect("no such a file"))
            .expect("unnable to create a Rope from reader"),
        ..Default::default()
    };
    let terminal = ratatui::init();
    let app_result = app.run(terminal);
    ratatui::restore();
    app_result
}
