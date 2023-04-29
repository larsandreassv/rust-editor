use crate::editor::buffer::{Buffer, atomic_command::BufferCommand};

pub struct ReplaceCharacter(pub char);

impl BufferCommand for ReplaceCharacter {
    fn execute(&mut self, buffer: &mut Buffer) {
        let index = buffer.cursor_buffer_index();
        buffer.buffer[index] = self.0;
    }

    fn execute_name(&mut self) -> String {
        format!("ReplaceCharacter({})", self.0)
    }
}