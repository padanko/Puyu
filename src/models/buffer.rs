pub struct ViewBuffer {
    pub cursor: usize,
    pub buffer: String,
    pub filename: String,
}

impl ViewBuffer {
    pub fn new(filename: &str) -> Self {
        Self {
            cursor: 0,
            buffer: String::new(),
            filename: filename.into(),
        }
    }

    /// 指定した位置に文字を挿入
    pub fn add_char(&mut self, c: char) {
        self.buffer.insert(self.cursor, c);
        self.cursor += c.len_utf8(); // 挿入した文字のバイト数だけカーソルを進める
    }

    /// 現在のカーソル位置の直前の文字を削除
    pub fn remove_char(&mut self) {
        if self.cursor == 0 || self.buffer.is_empty() {
            return; // カーソルが先頭の場合は何もしない
        }

        // 前の文字の開始位置を特定し、その位置を削除
        if let Some((prev_idx, _)) = self.buffer.char_indices().rev().find(|(i, _)| *i < self.cursor)
        {
            self.buffer.remove(prev_idx);
            self.cursor = prev_idx; // カーソルを更新
        }
    }

    /// カーソルを左に移動
    pub fn cur_move_left(&mut self) {
        if self.cursor == 0 {
            return; // カーソルが先頭の場合は何もしない
        }

        // 現在のカーソル位置より前の文字を探す
        if let Some((prev_idx, _)) = self.buffer.char_indices().rev().find(|(i, _)| *i < self.cursor)
        {
            self.cursor = prev_idx; // カーソルを更新
        }
    }

    /// カーソルを右に移動
    pub fn cur_move_right(&mut self) {
        // 現在のカーソル位置より後の文字を探す
        if let Some((next_idx, _)) = self
            .buffer
            .char_indices()
            .find(|(i, _)| *i > self.cursor)
        {
            self.cursor = next_idx; // 次の文字の開始位置に移動
        } else {
            // 文字列の最後の位置に移動（カーソルが末尾を指す）
            self.cursor = self.buffer.len();
        }
    }
}
