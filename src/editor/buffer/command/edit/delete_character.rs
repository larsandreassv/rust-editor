use crate::editor::buffer::{Buffer, atomic_command::BufferCommand};

pub struct DeleteCharacter;

impl BufferCommand for DeleteCharacter {
    fn execute(&mut self, buffer: &mut Buffer) {
        let index = buffer.cursor_buffer_index();
        buffer.buffer.remove(index);
    }

    fn execute_name(&mut self) -> String {
        "DeleteCharacter".to_string()
    }
}