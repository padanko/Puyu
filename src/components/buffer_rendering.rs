use crossterm::cursor;
use crossterm::execute;

use crate::components::render;
use crate::models::buffer;

pub async fn rendering(buffer: &buffer::ViewBuffer, start_lines: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();

    // ターミナルの縦サイズを取得します。
    let size_y = crossterm::terminal::size()?.1 as usize;

    // フッター用に一番下の行を確保
    let available_lines = size_y - 1; // 一番下の行をフッターに使うため、表示可能な行数は1行少なくなる

    // カーソルをリセットして、描画を開始します。
    render::cur_moveto(0, 0).await?;

    // バッファを行単位で分割します。
    let lines: Vec<&str> = buffer.buffer.split('\n').collect();

    if !lines.is_empty() {
        // 表示する行を計算します。
        let mut view_lines = lines.get(0..lines.len()).unwrap_or_default();
        if lines.len() > start_lines - 1 {
            view_lines = lines.get(start_lines..(start_lines + available_lines) - 1).unwrap_or_default();
        }

        // 各行を描画します。
        for line in view_lines {
            println!("{}", line);
            execute!(stdout, cursor::MoveToColumn(0))?;
        }
    }

    // 現在のカーソル位置を計算して設定します。
    let mut line_idx = 0;
    let mut col_idx = 0;
    let mut current_pos = 0;

    for (i, line) in lines.iter().enumerate() {
        let line_len = line.len();
        if current_pos + line_len >= buffer.cursor {
            line_idx = i;
            col_idx = buffer.cursor - current_pos;
            break;
        }
        current_pos += line_len + 1; // 各行の終端には改行があるため+1
    }

    // カーソルを移動します。フッターを除いた部分にカーソルを配置
    execute!(stdout, cursor::MoveTo(col_idx as u16, (line_idx - start_lines) as u16))?;

    Ok(())
}
