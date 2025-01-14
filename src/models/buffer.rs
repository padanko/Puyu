pub struct ViewBuffer {
    pub cursor: usize,
    pub buffer: String,
    pub filename: String,
}

impl ViewBuffer {
    pub fn new(filename: &str) -> ViewBuffer {
        Self { cursor: 0, buffer: "".into(), filename: filename.into() }
    }

    pub fn add_char(&mut self, c: char) {
        self.buffer.insert(self.cursor, c);
        self.cursor += 1;
    }

    pub fn remove_char(&mut self) {
        if !self.buffer.is_empty() {

            self.cursor = self.cursor.saturating_sub(1);
            self.buffer.remove(self.cursor);
        }
    }

    pub fn cur_move_left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }
    
    pub fn cur_move_right(&mut self) {
        if self.cursor < self.buffer.len() {
            self.cursor += 1;
        }
    }

    
}