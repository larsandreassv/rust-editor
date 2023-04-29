use crate::editor::buffer::{Buffer, command::BufferCommand};

pub struct SetEnvironmentVariable {
    pub key: Box<dyn EditorInfo>,
    pub value: Box<dyn EditorInfo>
}

impl BufferCommand for SetEnvironmentVariable {
    fn execute(&mut self, buffer: &mut Buffer) {
        todo!("SetEnvironmentVariable::execute")
    }

    fn execute_name(&mut self) -> String {
        format!("SetEnvironmentVariable({}, {})", self.key.execute_name(), self.value.execute_name())
    }
}