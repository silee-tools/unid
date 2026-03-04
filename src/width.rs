use unicode_width::UnicodeWidthChar;
use unicode_width::UnicodeWidthStr;

/// Returns the display width of a character in a monospace coding font.
/// Uses standard (non-CJK) width rules: CJK ideographs/hangul are 2,
/// but ambiguous-width characters (box-drawing, arrows) are 1.
pub fn char_width(ch: char) -> usize {
    ch.width().unwrap_or(0)
}

/// Returns the display width of a string in a monospace coding font.
pub fn str_width(s: &str) -> usize {
    s.width()
}

/// Pads a string to the given display width with trailing spaces.
/// If the string is already wider, it is returned as-is.
pub fn pad_to_width(s: &str, width: usize) -> String {
    let current = str_width(s);
    if current >= width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(width - current))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_char_width() {
        assert_eq!(char_width('A'), 1);
        assert_eq!(char_width(' '), 1);
    }

    #[test]
    fn cjk_char_width() {
        assert_eq!(char_width('한'), 2);
        assert_eq!(char_width('漢'), 2);
        assert_eq!(char_width('あ'), 2);
    }

    #[test]
    fn ascii_str_width() {
        assert_eq!(str_width("Hello"), 5);
    }

    #[test]
    fn cjk_str_width() {
        assert_eq!(str_width("한글"), 4);
        assert_eq!(str_width("漢字"), 4);
    }

    #[test]
    fn mixed_str_width() {
        assert_eq!(str_width("A한B"), 4); // 1 + 2 + 1
    }

    #[test]
    fn pad_to_width_ascii() {
        assert_eq!(pad_to_width("Hi", 5), "Hi   ");
    }

    #[test]
    fn pad_to_width_cjk() {
        // "한" is 2 wide, so pad to 5 needs 3 spaces
        assert_eq!(pad_to_width("한", 5), "한   ");
    }

    #[test]
    fn pad_to_width_already_wider() {
        assert_eq!(pad_to_width("Hello", 3), "Hello");
    }
}
