pub static VERSION: &str = "1.0";

pub use cursor::CursorController;
pub use editor::Editor;
pub use editor_contents::EditorContents;
pub use editor_rows::EditorRows;
pub use output::Output;
pub use reader::Reader;

mod cursor;
mod editor;
mod editor_contents;
mod editor_rows;
mod output;
mod reader;
