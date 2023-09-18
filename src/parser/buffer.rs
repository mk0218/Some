pub struct Buffer {
    pub content: String,
    pub state: BufferState,
}

pub enum BufferState {
    Idle,
    Space,
    Chars,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            content: String::new(),
            state: BufferState::Idle,
        }
    }

    pub fn push(&mut self, c: char) {
        self.content.push(c);
    }
}