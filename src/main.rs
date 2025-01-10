use crossterm::{cursor, style, terminal, execute};

use std::io;
use std::error;

fn header_draw(text: &str) -> Result<(), Box<dyn error::Error>>{
    // op = original position
    let ( op_x, op_y ) = cursor::position()?;
    let ( size_x, _ ) = terminal::size()?;

    execute!(io::stdout(), cursor::MoveTo(0,0))?;
    execute!(io::stdout(), style::SetBackgroundColor(style::Color::Red))?;
    execute!(io::stdout(), style::SetForegroundColor(style::Color::White))?;

    let repeat_count = ( size_x as usize - text.len() ) - 1;

    print!(" {}{}", text, " ".repeat(repeat_count));

    execute!(io::stdout(), style::SetBackgroundColor(style::Color::Reset))?;
    execute!(io::stdout(), style::SetForegroundColor(style::Color::Reset))?;

    execute!(io::stdout(), cursor::MoveTo(op_x, op_y))?;

    Ok(())
}

    // The beginning of a legendary application
fn main() -> Result<(), Box<dyn error::Error>> {

    terminal::enable_raw_mode()?;


    header_draw("Puyu")?;
    
    terminal::disable_raw_mode()?;
    Ok(())
}