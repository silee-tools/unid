/// A text object placed at a specific position.
#[derive(Debug, Clone)]
pub struct Text {
    pub col: usize,
    pub row: usize,
    pub content: String,
}

impl Text {
    pub fn new(col: usize, row: usize, content: impl Into<String>) -> Self {
        Self {
            col,
            row,
            content: content.into(),
        }
    }
}
