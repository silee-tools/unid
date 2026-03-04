/// Style for horizontal and vertical lines.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LineStyle {
    #[default]
    Light,
    Heavy,
    Double,
    Dash,
}

/// A horizontal line.
#[derive(Debug, Clone)]
pub struct HLine {
    pub col: usize,
    pub row: usize,
    pub length: usize,
    pub style: LineStyle,
}

impl HLine {
    pub fn new(col: usize, row: usize, length: usize) -> Self {
        Self {
            col,
            row,
            length,
            style: LineStyle::default(),
        }
    }
}

/// A vertical line.
#[derive(Debug, Clone)]
pub struct VLine {
    pub col: usize,
    pub row: usize,
    pub length: usize,
    pub style: LineStyle,
}

impl VLine {
    pub fn new(col: usize, row: usize, length: usize) -> Self {
        Self {
            col,
            row,
            length,
            style: LineStyle::default(),
        }
    }
}
