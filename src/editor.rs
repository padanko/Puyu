// テキストエディタ

use crossterm::{cursor, terminal, execute};

use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use crate::components;

pub fn run(code: &str) -> Result<(), Box<dyn std::error::Error>>{
    let mut cursor: usize = 0;
    let mut buffer = String::new();

    let mut filename = String::new();

    let mut add_mode = false;
    let mut escape_mode = false;

    for c in code.chars() {
        if add_mode {

            if c == '\\' {
                escape_mode = true;
            } else if !escape_mode && c == ';' {
                add_mode = false;    
            } else if escape_mode && c == 'n' {
                buffer.insert(cursor, '\n');
                escape_mode = false;
                cursor += 1;
            } else {
                buffer.insert(cursor, c);
                escape_mode = false;
                cursor += 1;
            }


        } else {
            
            if c == 'a' {
                add_mode = true;
            }   
            
            if c == 'v' {
                execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;
                components::header_draw("Puyu Editor")?;
                components::footer_draw("[OK]")?;
                execute!(io::stdout(),cursor::MoveTo(0,1))?;               
                components::print_(&buffer);
            }

            if c == 'f' && cursor < buffer.len(){
                cursor += 1;
            }

            if c == 'b' && cursor > 0 {
                cursor -= 1;
            }

            if c == 'r' {
                buffer.remove(cursor);
                cursor = cursor.saturating_sub(1);
            }

            if c == 's' {
                filename = buffer.clone();
                buffer = String::new();
                cursor = 0;
            }

            if c == 'l' {
                let mut file = fs::File::open(&mut filename)?;
                file.read_to_string(&mut buffer)?;
            }

            if c == 'S' {
                let mut file = fs::File::create(&mut filename)?;
                file.write_all(buffer.as_bytes())?;
            }

            if c == 'r' {
                buffer = String::new();
            }
        }
    }

    Ok(())
    
}