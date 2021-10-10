use crossterm::terminal;
use mini_editor_in_rust::Editor;
use mini_editor_in_rust::Output;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode");
        Output::clear_screen().expect("Error");
    }
}

fn main() -> crossterm::Result<()> {
    let _cleanup = CleanUp;
    terminal::enable_raw_mode()?;
    let mut editor = Editor::new();
    while editor.run()? {}
    Ok(())
}
