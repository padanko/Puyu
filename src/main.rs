use crossterm::terminal;
use crossterm::event;
use crossterm::cursor;
use crossterm::execute;

use std::error;
use std::time::Duration;
use std::io;

mod components;
mod defines;


fn fn_keyboard() -> Result<(), Box<dyn error::Error>> {
    let mut buffer = String::new();
    let mut mode = defines::KeyBoard_InputMode::default;
    let mut cursor = 0;
    loop {
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

    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;       
    execute!(io::stdout(), cursor::MoveTo(0,1))?;

    components::header_draw("Puyu")?;
    components::footer_draw("")?;

    fn_keyboard()?;

    Ok(())
}