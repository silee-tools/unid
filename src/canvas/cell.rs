use crate::width;

/// A single cell in the canvas grid, addressed by display-column.
#[derive(Debug, Clone)]
pub struct Cell {
    /// The character stored in this cell.
    pub ch: char,
    /// Display width of `ch` (1 for ASCII, 2 for CJK).
    pub display_width: usize,
    /// True if this cell is the continuation (right half) of a wide character.
    pub is_continuation: bool,
}

impl Cell {
    pub fn new(ch: char) -> Self {
        Self {
            ch,
            display_width: width::char_width(ch),
            is_continuation: false,
        }
    }

    pub fn space() -> Self {
        Self {
            ch: ' ',
            display_width: 1,
            is_continuation: false,
        }
    }

    pub fn continuation() -> Self {
        Self {
            ch: '\0',
            display_width: 0,
            is_continuation: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_cell() {
        let cell = Cell::new('A');
        assert_eq!(cell.ch, 'A');
        assert_eq!(cell.display_width, 1);
        assert!(!cell.is_continuation);
    }

    #[test]
    fn cjk_cell() {
        let cell = Cell::new('한');
        assert_eq!(cell.ch, '한');
        assert_eq!(cell.display_width, 2);
        assert!(!cell.is_continuation);
    }

    #[test]
    fn space_cell() {
        let cell = Cell::space();
        assert_eq!(cell.ch, ' ');
        assert_eq!(cell.display_width, 1);
    }

    #[test]
    fn continuation_cell() {
        let cell = Cell::continuation();
        assert!(cell.is_continuation);
        assert_eq!(cell.display_width, 0);
    }
}
