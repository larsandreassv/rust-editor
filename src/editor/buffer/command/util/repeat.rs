use crate::editor::buffer::{Buffer, atomic_command::BufferCommand};

pub struct Repeat(pub usize, pub Box<dyn BufferCommand>);

impl BufferCommand for Repeat {
    fn execute(&mut self, buffer: &mut Buffer) {
        for _ in 0..self.0 {
            self.1.execute(buffer);
        }
    }

    fn execute_name(&mut self) -> String {
        format!("Repeat({}, {})", self.0, self.1.execute_name())
    }
}