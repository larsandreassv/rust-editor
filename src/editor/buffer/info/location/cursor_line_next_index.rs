use crate::editor::buffer::{Buffer, command::BufferCommand};

pub struct CursorLineNextIndex;

impl EditorInfo for CursorLineNextIndex {
    fn execute(&mut self, buffer: &mut Buffer) -> String {
        let line = buffer.cursor.line;
        let index = buffer.cursor.index;
        let line_length = buffer.buffer[line].len();
        if index < line_length {
            buffer.cursor.index += 1;
        }
        buffer.cursor.index
    }
}