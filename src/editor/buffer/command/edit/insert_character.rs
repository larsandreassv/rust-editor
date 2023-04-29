use crate::editor::buffer::{Buffer, atomic_command::BufferCommand};

pub struct InsertCharacter(pub Box<dyn EditorInfo>);

impl BufferCommand for InsertCharacter {
    fn execute(&mut self, buffer: &mut Buffer) {
        let index = buffer.cursor_buffer_index();
        buffer.buffer.insert(index, self.0.read_info());
    }

    fn execute_name(&mut self) -> String {
        format!("InsertCharacter({})", self.0.read_info())
    }
}