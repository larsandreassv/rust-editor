use crate::editor::buffer::{Buffer, command::BufferCommand};

pub struct Write(String);

//write content of buffer to file or overwrite file if exists
impl Command<Buffer, ()> for Write {
    fn execute(&mut self, buffer: &mut Buffer) {
        let mut file = std::fs::File::create(&self.0).unwrap();
        file.write_all(buffer.buffer.as_bytes()).unwrap();
    }

    fn execute_name(&mut self) -> String {
        format!("Write({})", self.0)
    }
}