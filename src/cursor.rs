use crossterm::event::*;

pub struct CursorController {
    pub cursor_x: usize,
    pub cursor_y: usize,
    screen_cols: usize,
    screen_rows: usize,
    pub row_offset: usize,
}

impl CursorController {
    pub fn new(win_size: (usize, usize)) -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            screen_cols: win_size.0,
            screen_rows: win_size.1,
            row_offset: 0,
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode, number_of_rows: usize) {
        match direction {
            KeyCode::Up => self.cursor_y = self.cursor_y.saturating_sub(1),
            KeyCode::Left => self.cursor_x = self.cursor_x.saturating_sub(1),
            KeyCode::Down => {
                if self.cursor_y < number_of_rows {
                    self.cursor_y += 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_x != self.screen_cols - 1 {
                    self.cursor_x += 1;
                }
            }
            KeyCode::PageDown => self.cursor_y = self.screen_rows - 1,
            KeyCode::PageUp => self.cursor_y = 0,
            _ => unimplemented!(),
        }
    }

    pub fn scroll(&mut self) {
        // it means when cursor_y < row_offset, cursor will be unvisable
        // and we should change the row_offset to make cursor visable that is to scroll up
        self.row_offset = std::cmp::min(self.row_offset, self.cursor_y);
        // if cursor_y is below row_offset + screen_rows, it is unvisable
        // and we should scroll down
        if self.cursor_y >= self.row_offset + self.screen_rows {
            self.row_offset = self.cursor_y - self.screen_rows + 1;
        }
    }
}
