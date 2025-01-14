use crossterm::execute;
use crossterm::cursor;
use crossterm::style;
use crossterm::terminal;

use crate::models::colors;

// カーソル移動操作を簡略化
pub async fn cur_moveto(x: u16, y: u16) 
                    -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();

    execute!(stdout, cursor::MoveTo(x, y))?; // カーソル移動

    Ok(())
}


// カーソルの位置を取得する操作を簡略化
pub fn cur_getpos() -> Result<(u16, u16), Box<dyn std::error::Error>> {

    let (x, y) = cursor::position()?; // ここで取得する

    Ok((x, y))
}


// 文字の色を変更する
pub async fn change_fg_color(color: style::Color) 
                    -> Result<(), Box<dyn std::error::Error>>{
    
    let mut stdout = std::io::stdout();

    execute!(stdout, style::SetForegroundColor(color))?;

    Ok(())
}

// 背景の色を変更する
pub async fn change_bg_color(color: style::Color) 
                    -> Result<(), Box<dyn std::error::Error>>{
    
    let mut stdout = std::io::stdout();

    execute!(stdout, style::SetBackgroundColor(color))?;

    Ok(())
}


// フッターを描画する
// フッターの色についてはmodels/colors.rsにて記述されている
pub async fn render_footer(text: &str) 
        -> Result<(), Box<dyn std::error::Error>> {
    
    // 元のポジションを取得する
    // fp = from position
    let (fp_x, fp_y) = cur_getpos()?;


    let terminal_size = terminal::size()?; // ターミナルの縦横サイズを取得する
    let terminal_size_x = terminal_size.0; // 横のサイズを取得する
    let terminal_size_y = terminal_size.1; // 今回は縦のサイズを取得する
 
    cur_moveto(0, terminal_size_y-1).await?; //カーソルを左上に移動し
    let whitespace_length = ( terminal_size_x as usize - text.len() ) - 1; 
    // - 1をするのは左にも1つ空白が入るため

    change_bg_color(colors::FOOTER_BACKGROUND_COLOR).await?;
    change_fg_color(colors::FOOTER_FOREGROUND_COLOR).await?;

    let whitespaces = " ".repeat(whitespace_length);            
    print!(" {}{}", text, whitespaces);

    change_bg_color(style::Color::Reset).await?;
    change_fg_color(style::Color::Reset).await?;

    cur_moveto(fp_x, fp_y).await?;
        
    Ok(())
}