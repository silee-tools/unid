/// An arrow from one point to another.
/// Currently supports only horizontal and vertical arrows.
#[derive(Debug, Clone)]
pub struct Arrow {
    pub from_col: usize,
    pub from_row: usize,
    pub to_col: usize,
    pub to_row: usize,
}

impl Arrow {
    pub fn new(from_col: usize, from_row: usize, to_col: usize, to_row: usize) -> Self {
        Self {
            from_col,
            from_row,
            to_col,
            to_row,
        }
    }
}
