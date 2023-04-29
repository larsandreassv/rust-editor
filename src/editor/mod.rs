use self::{buffer::{Buffer, atomic_command::BufferCommand}, command::{buffer_command::BufferCommandEditorCommand}, mode::EditorMode};

pub mod buffer;
pub mod command;
pub mod mode;

pub struct Editor {
    pub current_buffer: Buffer,
    pub buffers: Vec<Buffer>,
    pub mode: EditorMode,
}

impl Editor {
    pub fn execute(&mut self, command: &mut Box<dyn BufferCommand>) {
        self.current_buffer.execute(command);
    }

    pub fn execute_name(&mut self, command: &mut Box<dyn BufferCommand>) -> String {
        self.current_buffer.execute_name(command)
    }
}

pub trait Command<TSource> {
    fn execute(&mut self, source: &mut TSource) -> Result<ExecutionResult<()>, String>;
}

pub trait InfoReader<TSource, TResult> {
    fn read_info(&mut self, source: &TSource) -> Result<ExecutionResult<TResult>, String>;
}

pub struct ExecutionResult<TResult> {
    pub result: TResult,
    pub command_name: String,
    pub verbose_command_name: String,
}