use crossterm::cursor;
use crossterm::event;
use crossterm::execute;
use crossterm::terminal;
use crossterm::ExecutableCommand;

use std::error;
use std::io;
use std::time::Duration;

mod components;
mod defines;

fn fn_keyboard() -> Result<(), Box<dyn error::Error>> {
    let mut buffer = String::new();
    let mut mode = defines::KeyboardInputmode::Default;
    let mut cursor = 0;
    loop {
        if mode == defines::KeyboardInputmode::Default {
            components::footer_draw("")?;
        }
        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(key_event) = event::read()? {
                components::keyboard_handle(key_event.code, &mut mode, &mut buffer, &mut cursor)?;
            }
        }
    }
}

// 伝説的なアプリケーションの幕開けだ...!
fn main() -> Result<(), Box<dyn error::Error>> {
    terminal::enable_raw_mode()?;

    io::stdout().execute(cursor::Hide)?;

    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;

    execute!(io::stdout(), cursor::MoveTo(0, 1))?;
    components::print_("Welcome to Puyu!\n\n");
    components::print_("[Commands]\n");
    components::print_("    |- :fo<filename>    a Simple File Viewer\n");
    components::print_("    '- :qu    quit.");

    execute!(io::stdout(), cursor::MoveTo(0, 0))?;
    components::header_draw("Puyu")?;
    components::footer_draw("")?;
    
    loop {

        if let Err(result) = fn_keyboard() {
            execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;
            execute!(io::stdout(), cursor::MoveTo(0, 1))?;
            components::print_("Welcome to Puyu!\n\n");
            components::print_("[Commands]\n");
            components::print_("    |- :fo<filename>    a Simple File Viewer\n");
            components::print_("    '- :qu    quit.");
        
            execute!(io::stdout(), cursor::MoveTo(0, 0))?;
            components::header_draw("Puyu")?;
            components::footer_draw(&format!("{:?}", result))?;
        }
    }
}
