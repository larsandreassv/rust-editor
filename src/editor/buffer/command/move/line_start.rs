use crate::editor::buffer::{Buffer, atomic_command::BufferCommand};

pub struct LineStart;

impl BufferCommand for LineStart {
    fn execute(&mut self, buffer: &mut Buffer) {
        buffer.cursor.column = 0;
    }

    fn execute_name(&mut self) -> String {
        "LineStart()".to_string()
    }
}