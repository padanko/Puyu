use crossterm::terminal;

use std::error;

mod components;

// The beginning of a legendary application
fn main() -> Result<(), Box<dyn error::Error>> {

    terminal::enable_raw_mode()?;


    components::header_draw("Puyu")?;



    terminal::disable_raw_mode()?;
    Ok(())
}