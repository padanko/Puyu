use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;

pub const KB_CTRL: KeyModifiers = KeyModifiers::CONTROL;
pub const KB_FILEOPEN: KeyCode = KeyCode::Char('x');
pub const KB_SAVE: KeyCode = KeyCode::Char('s');
// pub const KB_COPY: KeyCode = KeyCode::Char('c');
// pub const KB_PASTE: KeyCode = KeyCode::Char('v');
pub const KB_QUIT: KeyCode = KeyCode::Char('q');
pub const KB_PAGEUP: KeyCode = KeyCode::Up;
pub const KB_PAGEDOWN: KeyCode = KeyCode::Down;
