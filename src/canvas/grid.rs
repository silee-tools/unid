use crate::canvas::cell::Cell;
use crate::error::UnidError;
use crate::width;

/// A 2D grid of cells addressed by (display-column, row).
pub struct Canvas {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>,
}

impl Canvas {
    /// Creates a new canvas filled with spaces.
    pub fn new(width: usize, height: usize) -> Self {
        let grid = (0..height)
            .map(|_| (0..width).map(|_| Cell::space()).collect())
            .collect();
        Self {
            width,
            height,
            grid,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Places a single character at (col, row).
    /// For wide characters (CJK), also places a continuation cell at col+1.
    /// Returns error if the position is out of bounds.
    pub fn put_char(
        &mut self,
        col: usize,
        row: usize,
        ch: char,
        collision: bool,
    ) -> Result<(), UnidError> {
        let w = width::char_width(ch);

        if col + w > self.width || row >= self.height {
            return Err(UnidError::OutOfBounds {
                col,
                row,
                canvas_width: self.width,
                canvas_height: self.height,
            });
        }

        if collision {
            let existing = &self.grid[row][col];
            if existing.ch != ' ' && !existing.is_continuation {
                return Err(UnidError::Collision {
                    col,
                    row,
                    existing: existing.ch.to_string(),
                    incoming: ch.to_string(),
                });
            }
            if w == 2 {
                let next = &self.grid[row][col + 1];
                if next.ch != ' ' && !next.is_continuation {
                    return Err(UnidError::Collision {
                        col: col + 1,
                        row,
                        existing: next.ch.to_string(),
                        incoming: ch.to_string(),
                    });
                }
            }
        }

        self.grid[row][col] = Cell::new(ch);
        if w == 2 {
            self.grid[row][col + 1] = Cell::continuation();
        }
        Ok(())
    }

    /// Places a string starting at (col, row), advancing by each character's display width.
    pub fn put_str(
        &mut self,
        col: usize,
        row: usize,
        s: &str,
        collision: bool,
    ) -> Result<(), UnidError> {
        let mut current_col = col;
        for ch in s.chars() {
            self.put_char(current_col, row, ch, collision)?;
            current_col += width::char_width(ch);
        }
        Ok(())
    }

    /// Renders the canvas to a string.
    pub fn render(&self) -> String {
        let mut lines = Vec::with_capacity(self.height);
        for row in &self.grid {
            let mut line = String::new();
            for cell in row {
                if cell.is_continuation {
                    continue;
                }
                line.push(cell.ch);
            }
            lines.push(line.trim_end().to_string());
        }
        // Remove trailing empty lines
        while lines.last().is_some_and(|l| l.is_empty()) {
            lines.pop();
        }
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn empty_canvas() {
        let canvas = Canvas::new(5, 3);
        assert_eq!(canvas.render(), "");
    }

    #[test]
    fn put_ascii_char() {
        let mut canvas = Canvas::new(5, 3);
        canvas.put_char(0, 0, 'A', false).unwrap();
        assert_eq!(canvas.render(), "A");
    }

    #[test]
    fn put_cjk_char() {
        let mut canvas = Canvas::new(5, 3);
        canvas.put_char(0, 0, '한', false).unwrap();
        assert_eq!(canvas.render(), "한");
    }

    #[test]
    fn put_str_ascii() {
        let mut canvas = Canvas::new(10, 1);
        canvas.put_str(0, 0, "Hello", false).unwrap();
        assert_eq!(canvas.render(), "Hello");
    }

    #[test]
    fn put_str_cjk() {
        let mut canvas = Canvas::new(10, 1);
        canvas.put_str(0, 0, "한글", false).unwrap();
        assert_eq!(canvas.render(), "한글");
    }

    #[test]
    fn put_str_mixed() {
        let mut canvas = Canvas::new(10, 1);
        canvas.put_str(0, 0, "A한B", false).unwrap();
        assert_eq!(canvas.render(), "A한B");
    }

    #[test]
    fn cjk_continuation_cell() {
        let mut canvas = Canvas::new(4, 1);
        canvas.put_char(0, 0, '한', false).unwrap();
        // Column 1 should be a continuation cell
        assert!(canvas.grid[0][1].is_continuation);
        // Placing at column 2 should work
        canvas.put_char(2, 0, 'A', false).unwrap();
        assert_eq!(canvas.render(), "한A");
    }

    #[test]
    fn out_of_bounds() {
        let mut canvas = Canvas::new(3, 1);
        let result = canvas.put_char(3, 0, 'A', false);
        assert!(result.is_err());
    }

    #[test]
    fn cjk_out_of_bounds() {
        let mut canvas = Canvas::new(3, 1);
        // CJK at col 2 needs col 2 and 3, but width is 3 (cols 0-2)
        let result = canvas.put_char(2, 0, '한', false);
        assert!(result.is_err());
    }

    #[test]
    fn collision_detected() {
        let mut canvas = Canvas::new(5, 1);
        canvas.put_char(0, 0, 'A', true).unwrap();
        let result = canvas.put_char(0, 0, 'B', true);
        assert!(result.is_err());
    }

    #[test]
    fn collision_off_overwrites() {
        let mut canvas = Canvas::new(5, 1);
        canvas.put_char(0, 0, 'A', false).unwrap();
        canvas.put_char(0, 0, 'B', false).unwrap();
        assert_eq!(canvas.render(), "B");
    }

    #[test]
    fn multiline_render() {
        let mut canvas = Canvas::new(5, 3);
        canvas.put_str(0, 0, "Hello", false).unwrap();
        canvas.put_str(0, 2, "World", false).unwrap();
        assert_eq!(canvas.render(), "Hello\n\nWorld");
    }
}
