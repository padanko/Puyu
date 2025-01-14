use crossterm::execute;
use crossterm::cursor;


use crate::models::buffer;
use crate::components::render;

pub async fn rendering(buffer: &buffer::ViewBuffer, start_lines: usize) -> Result<(), Box<dyn std::error::Error>>{
    
    let mut stdout = std::io::stdout();

    // 縦のサイズを取得します。
    let size_y = crossterm::terminal::size()?.1 as usize;

    render::cur_moveto(0, 0).await?;

    let lines: Vec<&str> = buffer.buffer.split('\n').collect();
    

    // 0行ではない場合
    if lines.len() != 0 {
        
        let mut view_lines = lines.get(0..lines.len()).unwrap_or_default();
        if lines.len() > start_lines-1 {
            view_lines = lines.get(start_lines..(start_lines+size_y)-1).unwrap_or_default();
        }
        for line in view_lines {
            println!("{}", line);
            execute!(stdout, cursor::MoveToColumn(0))?;
        }
    }

    Ok(())
}