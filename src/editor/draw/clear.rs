use std::io::{Write, stdout};
use std::error::Error;
use crossterm::{event, terminal::{self, Clear, ClearType}, cursor::MoveTo};

pub fn clear() {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(MoveTo(0, 0))?;
    stdout.execute(Clear(ClearType::All))?;
    stdout.flush()?;
    terminal::disable_raw_mode()?;
    Ok(())
}