/// Border style for rectangles.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BorderStyle {
    #[default]
    Light,
    Heavy,
    Double,
    Rounded,
}

/// A rectangle with optional label.
#[derive(Debug, Clone)]
pub struct Rect {
    pub col: usize,
    pub row: usize,
    /// Inner width (excluding borders).
    pub width: usize,
    /// Inner height (excluding borders).
    pub height: usize,
    pub label: Option<String>,
    pub style: BorderStyle,
}

impl Rect {
    pub fn new(col: usize, row: usize, width: usize, height: usize) -> Self {
        Self {
            col,
            row,
            width,
            height,
            label: None,
            style: BorderStyle::default(),
        }
    }

    /// Total width including borders.
    pub fn outer_width(&self) -> usize {
        self.width + 2
    }

    /// Total height including borders.
    pub fn outer_height(&self) -> usize {
        self.height + 2
    }
}
