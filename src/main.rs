use crossterm::cursor;
use crossterm::event;
use crossterm::execute;
use crossterm::terminal;
use crossterm::ExecutableCommand;

use std::error;
use std::io;
use std::time::Duration;
use std::thread;

mod components;
mod defines;
mod alerm;
mod editor;

fn fn_keyboard() -> Result<(), Box<dyn error::Error>> {
    let mut buffer = String::new();
    let mut mode = defines::KeyboardInputmode::Default;
    let mut cursor = 0;
    loop {
        if event::poll(Duration::from_millis(0))? {
            if let event::Event::Key(key_event) = event::read()? {
                components::keyboard_handle(key_event.code, &mut mode, &mut buffer, &mut cursor)?;
            }
        }
    }
}

// 伝説的なアプリケーションの幕開けだ...!
fn main() -> Result<(), Box<dyn error::Error>> {

    println!("[\tOK\t]\t\tAlermSystem StartUP");

    terminal::enable_raw_mode()?;

    thread::spawn(|| {
        let _ = alerm::run_pending();
    });

    io::stdout().execute(cursor::Hide)?;

    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;
    execute!(io::stdout(), terminal::EnterAlternateScreen)?;
    execute!(io::stdout(), cursor::MoveTo(0, 1))?;
    components::print_("Welcome to Puyu!\n\n");
    components::print_("");

    components::header_draw("Puyu")?;
    components::footer_draw("")?;
    
    loop {

        if let Err(result) = fn_keyboard() {
            execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;
            execute!(io::stdout(), cursor::MoveTo(0, 1))?;
        
            components::header_draw("Puyu")?;
            components::footer_draw(&format!("{:?}", result))?;
        }
    }
}
