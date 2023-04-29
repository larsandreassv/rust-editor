use crate::editor::{Editor, buffer::{atomic_command::BufferCommand, Buffer}, Command};

pub struct BufferCommandEditorCommand(pub Box<dyn Command<Buffer, ()>>);

impl Command<Editor, ()> for BufferCommandEditorCommand {
    fn execute(&mut self, editor: &mut Editor) {
        self.0.execute(&mut editor.current_buffer);
    }
}