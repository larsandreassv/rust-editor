use crate::editor::buffer::{Buffer, command::BufferCommand};

pub struct LineNext;

impl BufferCommand for LineNext {
    fn execute(&mut self, buffer: &mut Buffer) {
        if buffer.cursor.line < buffer.buffer.len() - 1 {
            buffer.cursor.line += 1;
        }
    }

    fn execute_name(&mut self) -> String {
        "LineNext()".to_string()
    }
}