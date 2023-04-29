use crate::editor::buffer::{Buffer, command::BufferCommand};

pub struct LinePrevious;

impl BufferCommand for LinePrevious {
    fn execute(&mut self, buffer: &mut Buffer) {
        if buffer.cursor.line > 0 {
            buffer.cursor.line -= 1;
        }
    }

    fn execute_name(&mut self) -> String {
        "LinePrevious()".to_string()
    }
}