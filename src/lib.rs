pub static VERSION: &str = "1.0";

pub use cursor::CursorController;
pub use editor::Editor;
pub use editor_contents::EditorContents;
pub use output::Output;
pub use reader::Reader;

mod cursor;
pub mod editor;
mod editor_contents;
pub mod output;
pub mod reader;
