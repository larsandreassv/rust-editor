use crate::editor::buffer::{Buffer, command::BufferCommand};

pub struct Jump(pub usize, pub usize);

impl BufferCommand for Jump {
    fn execute(&mut self, buffer: &mut Buffer) {
        buffer.cursor.line = self.0;
        buffer.cursor.column = self.1;
    }

    fn execute_name(&mut self) -> String {
        format!("Jump({}, {})", self.0, self.1)
    }
}