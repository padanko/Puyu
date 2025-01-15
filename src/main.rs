use components::initial::exit;
use models::{buffer, keybinds};
use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

use std::sync::Arc;
use std::io::stdout;

use crossterm::event;
use crossterm::terminal;
use crossterm::execute;

mod models;
mod components;

// キーバインドについてはmodels/keybinds.rsを参照
async fn keyboard_fn(
    buffers: Arc<Mutex<Vec<buffer::ViewBuffer>>>
) -> Result<(), Box<dyn std::error::Error>> {

    let mut lines = 0 ;

    let mut selected_buffer = 0;


    loop {
        if event::poll(tokio::time::Duration::from_millis(500))? {
            if let event::Event::Key(event::KeyEvent { code, modifiers, .. }) = event::read()? {
                let mut buffers = buffers.lock().await;
                let buffers_length = buffers.len();
                if let Some(mut buffer) = buffers.get_mut(selected_buffer) {
                   
                    if modifiers == keybinds::KB_CTRL {
                        match code {
                            keybinds::KB_QUIT => {
                                exit().await?;
                                break;
                            },
                            keybinds::KB_SAVE => {
                                // ファイルを保存します。
                                let mut fp = tokio::fs::File::create(&buffer.filename).await?;
                                fp.write_all(&buffer.buffer.as_bytes()).await?;
                            },
                            keybinds::KB_FILEOPEN => {
                                // ファイルを保存します。
                                let mut fp = tokio::fs::File::open(&buffer.buffer).await?;
                                buffer.filename = buffer.buffer.clone();
                                buffer.buffer = "".into();
                                fp.read_to_string(&mut buffer.buffer).await?;
                            },
                            event::KeyCode::Left => {
                                for _ in 0..5 {
                                    buffer.cur_move_left();
                                }
                            },
                            event::KeyCode::Right => {
                                for _ in 0..5 {
                                    buffer.cur_move_right();
                                }
                            },
                            keybinds::KB_SELECT_BUFFER_LEFT => {
                                selected_buffer = selected_buffer.saturating_sub(1);
                                buffer = buffers.get_mut(selected_buffer).unwrap();
                            },
                            keybinds::KB_SELECT_BUFFER_RIGHT => {
                                if selected_buffer < buffers_length-1 {
                                    selected_buffer += 1;
                                }
                               buffer =  buffers.get_mut(selected_buffer).unwrap();
                            },
                            _ => {}
                        }
                    }
                    else {
                        match code {
                            event::KeyCode::Char(c) => {
                                buffer.add_char(c);
                            }
                            event::KeyCode::Enter => {
                                buffer.add_char('\n');
                            }
                            event::KeyCode::Left => {
                                buffer.cur_move_left();
                            }
                            event::KeyCode::Right => {
                                buffer.cur_move_right();
                            }

                            event::KeyCode::Backspace => {
                                buffer.remove_char();
                            },
                            event::KeyCode::Down => {
                                let max_line = buffer.buffer.split('\n').collect::<Vec<&str>>().len();

                                if lines < max_line {
                                    lines += 1;
                                }
                            },
                            event::KeyCode::Up => {
                                lines = lines.saturating_sub(1);
                            }
                            _ => {}
                        }
                    }
                    
                    // レンダリング
                    execute!(stdout(), terminal::Clear(terminal::ClearType::All))?; // 全部消す
                    components::buffer_rendering::rendering( buffer, lines).await?;
                    components::render::render_footer(&format!("Puyu Editor   -   {}", buffer.filename)).await?;
                                        
                }
            }
        }
    }
    Ok(())
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // 標準出力をスレッドセーフに扱えるようにする

    // ファイルの編集に使うバッファー
    let mut buffers: Vec<models::buffer::ViewBuffer> = Vec::new();
    
    for i in 0..10 {
        buffers.push(buffer::ViewBuffer::new(&format!("tmp{}.txt", i+1)));
    }

    // Rawモードを起動したりカーソルの位置を変えたりする
    components::initial::start().await?;

    components::render::render_footer("Welcome to Puyu Editor").await?;

    if let Err(_) = keyboard_fn(Arc::new(Mutex::new(buffers))).await {

        components::initial::exit().await?;

    }
    
    Ok(())

}