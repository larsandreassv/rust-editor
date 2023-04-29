use super::Command;

pub mod atomic_command;

pub struct Buffer {
    pub buffer: Vec<char>,
    pub cursor: Cursor,
    pub new_line_char_index: Vec<usize>,
}

pub struct Cursor {
    pub line: usize,
    pub column: usize,
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
            line: 0,
            column: 0,
        }
    }
}

impl Buffer {
    pub fn execute(&mut self, command: &mut Box<dyn Command<Buffer, ()>>) {
        command.execute(self);
    }

    pub fn execute_name(&mut self, command: &mut Box<dyn Command<Buffer, ()>>) -> String {
        command.execute_name()
    }

    fn cursor_buffer_index(&self) -> usize {
        self.new_line_char_index[self.cursor.line] + self.cursor.column
    }
}