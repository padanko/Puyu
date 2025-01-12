use crossterm::{cursor, event, execute, style, terminal};

use crossterm::ExecutableCommand;

use std::error;
use std::fs;
use std::io::{self, Read};
use std::process;

use crate::defines;

pub fn exit(code: i32, message: &str) -> Result<(), Box<dyn error::Error>> {
    terminal::disable_raw_mode()?;
    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;
    execute!(io::stdout(), cursor::MoveTo(0, 0))?;
    io::stdout().execute(cursor::Show)?;
    println!("{}", message);
    process::exit(code);
}

pub fn header_draw(text: &str) -> Result<(), Box<dyn error::Error>> {
    // op = original position, もともとの位置
    let (op_x, op_y) = cursor::position()?;
    let (size_x, _) = terminal::size()?;

    execute!(io::stdout(), cursor::MoveTo(0, 0))?;

    execute!(io::stdout(), style::SetBackgroundColor(style::Color::Red))?;
    execute!(io::stdout(), style::SetForegroundColor(style::Color::White))?;

    let repeat_count = (size_x as usize - text.len()) - 1;

    print!(" {}{}", text, " ".repeat(repeat_count));

    execute!(io::stdout(), style::SetBackgroundColor(style::Color::Reset))?;
    execute!(io::stdout(), style::SetForegroundColor(style::Color::Reset))?;

    execute!(io::stdout(), cursor::MoveTo(op_x, op_y))?;

    Ok(())
}

pub fn footer_draw(text: &str) -> Result<(), Box<dyn error::Error>> {
    // op = original position, もともとの位置
    let (op_x, op_y) = cursor::position()?;
    let (size_x, size_y) = terminal::size()?;

    execute!(io::stdout(), cursor::MoveTo(0, size_y - 1))?;

    execute!(io::stdout(), style::SetBackgroundColor(style::Color::White))?;
    execute!(io::stdout(), style::SetForegroundColor(style::Color::Black))?;

    let repeat_count = (size_x as usize - text.len()) - 1;

    print!(" {}{}", text, " ".repeat(repeat_count));

    execute!(io::stdout(), style::SetBackgroundColor(style::Color::Reset))?;
    execute!(io::stdout(), style::SetForegroundColor(style::Color::Reset))?;

    execute!(io::stdout(), cursor::MoveTo(op_x, op_y))?;

    Ok(())
}

pub fn keyboard_handle(
    kc: event::KeyCode,
    mode: &mut defines::KeyboardInputmode,
    buffer: &mut String,
    cursor: &mut usize,
) -> Result<(), Box<dyn error::Error>> {
    match mode {
        defines::KeyboardInputmode::Default => {
            if let event::KeyCode::Char(':') = kc {
                *mode = defines::KeyboardInputmode::Command;
                header_draw("Puyu [ Command ]")?;
                footer_draw(":")?;
            }
        }

        defines::KeyboardInputmode::Command => {
            match kc {
                event::KeyCode::Enter => {
                    header_draw("Puyu")?;
                    *mode = defines::KeyboardInputmode::Default;
                    command_process(buffer)?;
                    *buffer = "".into();
                    *cursor = 0;
                }
                event::KeyCode::Backspace => {
                    if !buffer.is_empty() {
                        buffer.pop();
                        *cursor -= 1;
                    }
                }
                event::KeyCode::Char(c) => {
                    buffer.insert(*cursor, c);
                    *cursor += 1;
                }
                _ => {}
            }
            footer_draw(&format!(":{}", &buffer))?;
        }
    }

    Ok(())
}

pub fn command_process(command: &str) -> Result<(), Box<dyn error::Error>> {
    if command.len() >= 2 {
        let initial: Vec<char> = command.chars().take(2).collect();
        let body: String = command.chars().skip(2).collect();

        if initial == vec!['q', 'u'] {
            exit(0, "quit")?;
        }

        if initial == vec!['f', 'o'] {
            execute!(io::stdout(), cursor::MoveTo(0, 1))?;

            execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;
            header_draw(&format!("Puyu [File \"{}\" Opened]", &body))?;
            footer_draw("")?;

            let mut file = fs::File::open(&body)?;
            let mut buffer = String::new();

            file.read_to_string(&mut buffer)?;

            print_(&buffer);
        }
    }
    Ok(())
}

pub fn print_(text: &str) {
    for c in text.chars() {
        if c == '\n' {
            let _ = execute!(io::stdout(), cursor::MoveDown(1));
            let _ = execute!(io::stdout(), cursor::MoveToColumn(0));
        } else {
            print!("{}", c);
        }
    }
}
