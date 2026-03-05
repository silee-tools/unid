use crate::object::rect::Side;
use crate::width;

/// A text object placed at a specific position.
#[derive(Debug, Clone)]
pub struct Text {
    pub col: usize,
    pub row: usize,
    pub content: String,
    pub id: Option<String>,
}

impl Text {
    pub fn new(col: usize, row: usize, content: impl Into<String>) -> Self {
        Self {
            col,
            row,
            content: content.into(),
            id: None,
        }
    }

    /// Bounding box width (max line width).
    fn bbox_width(&self) -> usize {
        self.content
            .lines()
            .map(width::str_width)
            .max()
            .unwrap_or(0)
    }

    /// Bounding box height (number of lines).
    fn bbox_height(&self) -> usize {
        self.content.lines().count().max(1)
    }

    /// Source anchor: 1 cell outside bounding box (arrow starts here).
    pub fn src_anchor(&self, side: Side) -> (usize, usize) {
        let w = self.bbox_width();
        let h = self.bbox_height();
        match side {
            Side::Top => (self.col + w / 2, self.row.saturating_sub(1)),
            Side::Bottom => (self.col + w / 2, self.row + h),
            Side::Left => (self.col.saturating_sub(1), self.row + h / 2),
            Side::Right => (self.col + w, self.row + h / 2),
        }
    }

    /// Dest anchor: 1 cell OUTSIDE bounding box (arrowhead does not overwrite content).
    pub fn dst_anchor(&self, side: Side) -> (usize, usize) {
        let w = self.bbox_width();
        let h = self.bbox_height();
        match side {
            Side::Top => (self.col + w / 2, self.row.saturating_sub(1)),
            Side::Bottom => (self.col + w / 2, self.row + h),
            Side::Left => (self.col.saturating_sub(1), self.row + h / 2),
            Side::Right => (self.col + w, self.row + h / 2),
        }
    }
}
