use crossterm::execute;
use crossterm::terminal;

use crate::components::render;

// プログラム開始時に呼び出される
pub async fn start() -> Result<(), Box<dyn std::error::Error>> {

    let mut stdout = std::io::stdout();

    terminal::enable_raw_mode()?; // rawモードを有効にする
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?; // 全部消す
    render::cur_moveto(0, 1).await?; // カーソルをx=0, y=1に移動

    Ok(())

}


// プログラム終了時に呼び出される
pub async fn exit() -> Result<(), Box<dyn std::error::Error>> {

    let mut stdout = std::io::stdout();


    terminal::disable_raw_mode()?; // rawモードを無効にする
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?; // 全部消す
    render::cur_moveto(0, 0).await?; // カーソルの位置をx, yともに0にする

    Ok(())

}