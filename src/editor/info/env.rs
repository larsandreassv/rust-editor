use crate::editor::buffer::{Buffer, command::BufferCommand};

pub struct Env(Box<dyn EditorInfo>);

impl EditorInfo for Env {
   fn read_info(&mut self, editor: &mut Editor) -> Result<EditorInfoResult, String> {
       let result = std::env::var(&self.0).unwrap_or_else(|_| "".to_string());
       Ok(EditorInfoResult {
           result,
           command_name: format!("Env({} -> {})", self.0.command_name, self.0.result),
           verbose_command_name: format!("Env({} -> {})", self.0.verbose_command_name, self.0.result),
       })
   }
}