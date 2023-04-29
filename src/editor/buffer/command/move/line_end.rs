use crate::editor::buffer::{Buffer, command::BufferCommand};

pub struct LineEnd;

impl BufferCommand for LineEnd {
    fn execute(&mut self, buffer: &mut Buffer) {
        todo!("LineEnd::execute")
    }

    fn execute_name(&mut self) -> String {
        Jump(Cursor)
    }
}