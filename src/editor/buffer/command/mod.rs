use super::Buffer;

pub mod edit;
pub mod r#move;
pub mod util;

pub trait BufferCommand {
    fn execute(&mut self, buffer: &mut Buffer);
    fn execute_name(&mut self) -> String;
}