// mod editor;

// use crate::editor::buffer::atomic_command::edit::insert_character::InsertCharacter;
// use crate::editor::buffer::atomic_command::BufferCommand;
// use crossterm::event::{Event, KeyCode};
// use editor::buffer::Buffer;
// use editor::Editor;

// struct LineEnd;

// impl BufferCommand for LineEnd {
//     fn execute(&mut self, buffer: &mut Buffer) {
//         buffer.cursor = buffer.buffer.len();
//     }
// }

// struct NextCharacter;

// impl BufferCommand for NextCharacter {
//     fn execute(&mut self, buffer: &mut Buffer) {
//         buffer.cursor += 1;
//     }
// }

// struct PreviousCharacter;

// impl BufferCommand for PreviousCharacter {
//     fn execute(&mut self, buffer: &mut Buffer) {
//         buffer.cursor -= 1;
//     }
// }

// struct NextLine;
// struct PreviousLine;
// struct FileStart;
// struct FileEnd;
// struct NextWord;
// struct PreviousWord;
// struct NextParagraph;
// struct PreviousParagraph;
// struct NextCharacterSearch(char);
// struct PreviousCharacterSearch(char);
// struct NextWordSearch(String);
// struct PreviousWordSearch(String);
// struct NextRegexSearch(String);
// struct PreviousRegexSearch(String);

// fn main() {
//     let mut editor = Editor {
//         buffer: Buffer {
//             buffer: vec!['a', 'b', 'c'],
//             cursor: 0,
//         },
//         mode: EditorMode::Normal,
//     };

//     // Send insertCharacterEditBufferCommand
//     let mut editor_command: Box<dyn BufferCommand> = Box::new(InsertCharacter('d'));

//     println!("{:?}", editor_command.execute_name());
// }
// use std::io::{Write, stdout};
// use std::error::Error;
// use crossterm::{event, terminal::{self, Clear, ClearType}, cursor::MoveTo};

// fn main() -> Result<(), Box<dyn Error>> {
//     // Set terminal to raw mode
//     terminal::enable_raw_mode()?;

//     // Create a buffer to store the text
//     let mut buffer: Vec<char> = vec![];

//     loop {
//         // Clear the terminal
//         print!("{}", Clear(ClearType::All));
//         stdout().flush()?;

//         let mut x = 0;
//         let mut y = 0;
//         // Print the buffer to the screen
//         for (i, c) in buffer.iter().enumerate() {
//             if *c == '\n' {
//                 x = 0;
//                 y += 1;
//                 continue;
//             }
//             print!("{}", MoveTo(x as u16, y as u16));
//             print!("{}", c);

//             x += 1;
//         }

//         // Print the cursor position
//         print!("{}", MoveTo(x as u16, y as u16));

//         stdout().flush()?;

//         // Wait for an event
//         if let Event::Key(event) = event::read()? {
//             match event.code {
//                 KeyCode::Char(c) => {
//                     // Insert character into buffer
//                     buffer.push(c);
//                 }
//                 KeyCode::Backspace => {
//                     // Remove character from buffer
//                     buffer.pop();
//                 }
//                 KeyCode::Enter => {
//                     // Insert newline character into buffer
//                     buffer.push('\n');
//                 }
//                 KeyCode::Esc => {
//                     // Exit the loop
//                     break;
//                 }
//                 _ => {}
//             }
//         }
//     }

//     // Restore terminal settings
//     terminal::disable_raw_mode()?;

//     Ok(())
// }

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::Print,
    terminal,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen, SetSize,
    },
};
use std::io::{self, stdout, Write};
use std::time::Duration;

fn main() {
    let items = vec!["Option 1", "Option 2", "Option 3", "Option 4"];
    let selected_item = select_item(&items);
    match selected_item {
        Ok(item) => println!("You selected: {}", item),
        Err(_) => println!("You didn't select anything!"),
    }
}

fn select_item<'a>(items: &'a Vec<&'a str>) -> io::Result<&'a str> {
    enable_raw_mode()?; // Enable raw mode to capture key events
    // terminal::enable_mouse_capture()?; // Enable mouse capture for mouse events

    let mut selected_index = 0;

    // Enter alternate screen buffer to draw on a smaller window
    execute!(stdout(), EnterAlternateScreen, SetSize(40, 10))?;

    loop {
        draw_dropdown(&items, selected_index)?;

        // Handle key events
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        break; // Exit the loop on 'q' or Esc key press
                    }
                    KeyCode::Up => {
                        selected_index = (selected_index + items.len() - 1) % items.len();
                    }
                    KeyCode::Down => {
                        selected_index = (selected_index + 1) % items.len();
                    }
                    KeyCode::Enter => {
                        break; // Exit the loop on Enter key press
                    }
                    _ => {} // Ignore other key events
                }
            }
        }
    }

    // return the selected item
    disable_raw_mode()?; // Disable raw mode

    return Ok(items[selected_index]);
}

fn draw_dropdown(items: &[&str], selected_index: usize) -> io::Result<()> {
    let terminal_size = terminal::size()?;
    let center_x = terminal_size.0 / 2;
    let center_y = terminal_size.1 / 2;

    let dropdown_width = 20;
    let dropdown_height = items.len() + 2;
    let dropdown_start_x = center_x - dropdown_width / 2;
    let dropdown_start_y = center_y as u16 - dropdown_height as u16 / 2;

    // Clear the section which the dropdown occupies
    execute!(
        stdout(),
        MoveTo(dropdown_start_x, dropdown_start_y),
        Clear(ClearType::UntilNewLine),
        MoveTo(dropdown_start_x, dropdown_start_y + 1),
        Clear(ClearType::UntilNewLine),
        MoveTo(dropdown_start_x, dropdown_start_y + 2),
        Clear(ClearType::UntilNewLine),
    )?;

    for (index, item) in items.iter().enumerate() {
        let item_x = dropdown_start_x + 1;
        let item_y = dropdown_start_y + 1 + index as u16;

        if index == selected_index {
            // Highlight the selected item
            execute!(
                stdout(),
                MoveTo(item_x, item_y),
                Print(format!("> {}", item))
            )?;
        } else {
            execute!(
                stdout(),
                MoveTo(item_x, item_y),
                Print(format!(" {}", item))
            )?;
        }
    }
    // Reset cursor position to bottom right corner
    execute!(stdout(), MoveTo(terminal_size.0, terminal_size.1), Hide)?;

    stdout().flush()?; // Flush stdout to update the terminal

    Ok(())
}

// Line(PreviousLine -> 2)
// Repeat(PreviousLine -> 2, NextCharacter())
// Line(ReadInteger -> 123)
// Repeat(123, Line(NextLineIndex -> 32))
// Line(FirstNr(ChatGpt("return line nr of problem", GithubIssueBody(Env("SoftRigGithubURL") -> "http://asfdsadf.com", CurrentIssueId -> 32))))
// Compare(EditGptFix(GithubIssueBody(Env("SoftRigGithubURL") -> "http://asfdsadf.com", CurrentIssueId -> 32) -> "Color should be red instead of green")))
// Line(NextLineIndex -> 33)
// Print(NextLineIndex - 33)
// Paste(NextLineIndex -> 34)
// Copy(NextLineIndex -> 42)
