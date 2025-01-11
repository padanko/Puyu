use crossterm::{cursor, style, terminal, event, execute};

use std::io;
use std::error;
use std::process;

use crate::defines;

pub fn header_draw(text: &str) -> Result<(), Box<dyn error::Error>> {
    // op = original position, もともとの位置
    let ( op_x, op_y ) = cursor::position()?;
    let ( size_x, _ ) = terminal::size()?;

    execute!(io::stdout(), cursor::MoveTo(0,0))?;

    execute!(io::stdout(), style::SetBackgroundColor(style::Color::White))?;
    execute!(io::stdout(), style::SetForegroundColor(style::Color::Black))?;

    let repeat_count = ( size_x as usize - text.len() ) - 1;

    print!(" {}{}", text, " ".repeat(repeat_count));

    execute!(io::stdout(), style::SetBackgroundColor(style::Color::Reset))?;
    execute!(io::stdout(), style::SetForegroundColor(style::Color::Reset))?;

    execute!(io::stdout(), cursor::MoveTo(op_x, op_y))?;

    Ok(())
}


pub fn footer_draw(text: &str) -> Result<(), Box<dyn error::Error>> {
    // op = original position, もともとの位置
    let ( op_x, op_y ) = cursor::position()?;
    let ( size_x, size_y ) = terminal::size()?;

    execute!(io::stdout(), cursor::MoveTo(0,size_y-1))?;

    execute!(io::stdout(), style::SetBackgroundColor(style::Color::White))?;
    execute!(io::stdout(), style::SetForegroundColor(style::Color::Black))?;

    let repeat_count = ( size_x as usize - text.len() ) - 1;

    print!(" {}{}", text, " ".repeat(repeat_count));

    execute!(io::stdout(), style::SetBackgroundColor(style::Color::Reset))?;
    execute!(io::stdout(), style::SetForegroundColor(style::Color::Reset))?;

    execute!(io::stdout(), cursor::MoveTo(op_x, op_y))?;

    Ok(())
}


pub fn keyboard_handle(kc: event::KeyCode, mode: &mut defines::KeyBoard_InputMode, 
    buffer: &mut String, cursor: &mut usize) -> Result<(), Box<dyn error::Error>> {


    match mode {
        defines::KeyBoard_InputMode::default => {
            match kc {
                event::KeyCode::Char(':') => {
                    *mode = defines::KeyBoard_InputMode::command;
                    header_draw("Puyu [ Command ]")?;
                },
                _ => { }
            }
        },

        defines::KeyBoard_InputMode::command => {
            match kc {
                event::KeyCode::Enter => {
                    header_draw("Puyu")?;
                    *mode = defines::KeyBoard_InputMode::default;
                    command_process(&buffer)?;
                    *buffer = "".into();
                    *cursor = 0;
                },
                event::KeyCode::Backspace => {
                    if buffer.len() > 0 {
                        buffer.pop();
                        *cursor -= 1;
                    }
                },
                event::KeyCode::Char(c) => {
                    buffer.insert(cursor.clone(), c);
                    *cursor += 1;
                }
                _ => { }
            }
            footer_draw(&format!(":{}", &buffer))?;


        } 
    }


    Ok(())

}


pub fn command_process(command: &str) -> Result<(), Box<dyn error::Error>>{

    if command == "quit" {
        terminal::disable_raw_mode()?;
        execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;       
        process::exit(1);
    }
    Ok(())

}