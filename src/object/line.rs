use crate::object::rect::Side;

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
    pub id: Option<String>,
    pub legend: Option<super::Legend>,
}

impl HLine {
    pub fn new(col: usize, row: usize, length: usize) -> Self {
        Self {
            col,
            row,
            length,
            style: LineStyle::default(),
            id: None,
            legend: None,
        }
    }

    /// Source anchor: 1 cell outside the line (arrow starts here).
    pub fn src_anchor(&self, side: Side) -> (usize, usize) {
        let mid = self.col + self.length / 2;
        match side {
            Side::Top => (mid, self.row.saturating_sub(1)),
            Side::Bottom => (mid, self.row + 1),
            Side::Left => (self.col.saturating_sub(1), self.row),
            Side::Right => (self.col + self.length, self.row),
        }
    }

    /// Dest anchor: 1 cell OUTSIDE the line (arrowhead does not overwrite line).
    pub fn dst_anchor(&self, side: Side) -> (usize, usize) {
        let mid = self.col + self.length / 2;
        match side {
            Side::Top => (mid, self.row.saturating_sub(1)),
            Side::Bottom => (mid, self.row + 1),
            Side::Left => (self.col.saturating_sub(1), self.row),
            Side::Right => (self.col + self.length, self.row),
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
    pub id: Option<String>,
    pub legend: Option<super::Legend>,
}

impl VLine {
    pub fn new(col: usize, row: usize, length: usize) -> Self {
        Self {
            col,
            row,
            length,
            style: LineStyle::default(),
            id: None,
            legend: None,
        }
    }

    /// Source anchor: 1 cell outside the line (arrow starts here).
    pub fn src_anchor(&self, side: Side) -> (usize, usize) {
        let mid = self.row + self.length / 2;
        match side {
            Side::Top => (self.col, self.row.saturating_sub(1)),
            Side::Bottom => (self.col, self.row + self.length),
            Side::Left => (self.col.saturating_sub(1), mid),
            Side::Right => (self.col + 1, mid),
        }
    }

    /// Dest anchor: 1 cell OUTSIDE the line (arrowhead does not overwrite line).
    pub fn dst_anchor(&self, side: Side) -> (usize, usize) {
        let mid = self.row + self.length / 2;
        match side {
            Side::Top => (self.col, self.row.saturating_sub(1)),
            Side::Bottom => (self.col, self.row + self.length),
            Side::Left => (self.col.saturating_sub(1), mid),
            Side::Right => (self.col + 1, mid),
        }
    }
}
