use crate::canvas::Canvas;
use crate::error::UnidError;
use crate::object::{Arrow, BorderStyle, DrawObject, HLine, LineStyle, Rect, Text, VLine};
use crate::width;

/// Renders DrawObjects onto a Canvas.
pub struct Renderer {
    pub canvas: Canvas,
    pub collision: bool,
}

impl Renderer {
    pub fn new(canvas: Canvas, collision: bool) -> Self {
        Self { canvas, collision }
    }

    /// Draws a single object onto the canvas.
    pub fn draw(&mut self, object: &DrawObject) -> Result<(), UnidError> {
        match object {
            DrawObject::Rect(r) => self.draw_rect(r),
            DrawObject::Text(t) => self.draw_text(t),
            DrawObject::HLine(h) => self.draw_hline(h),
            DrawObject::VLine(v) => self.draw_vline(v),
            DrawObject::Arrow(a) => self.draw_arrow(a),
        }
    }

    /// Draws all objects in order.
    pub fn draw_all(&mut self, objects: &[DrawObject]) -> Result<(), UnidError> {
        for obj in objects {
            self.draw(obj)?;
        }
        Ok(())
    }

    /// Renders the canvas to string output.
    pub fn render(&self) -> String {
        self.canvas.render()
    }

    fn draw_rect(&mut self, rect: &Rect) -> Result<(), UnidError> {
        let (tl, tr, bl, br, h, v) = border_chars(rect.style);
        let col = rect.col;
        let row = rect.row;
        let inner_w = rect.width;
        let inner_h = rect.height;

        // Top border: ┌───┐
        self.canvas.put_char(col, row, tl, self.collision)?;
        for c in 1..=inner_w {
            self.canvas.put_char(col + c, row, h, self.collision)?;
        }
        self.canvas
            .put_char(col + inner_w + 1, row, tr, self.collision)?;

        // Side borders and interior
        for r in 1..=inner_h {
            self.canvas.put_char(col, row + r, v, self.collision)?;
            self.canvas
                .put_char(col + inner_w + 1, row + r, v, self.collision)?;
        }

        // Bottom border: └───┘
        self.canvas
            .put_char(col, row + inner_h + 1, bl, self.collision)?;
        for c in 1..=inner_w {
            self.canvas
                .put_char(col + c, row + inner_h + 1, h, self.collision)?;
        }
        self.canvas
            .put_char(col + inner_w + 1, row + inner_h + 1, br, self.collision)?;

        // Label (centered in first middle row)
        if let Some(label) = &rect.label {
            let label_w = width::str_width(label);
            if label_w <= inner_w {
                let pad_left = (inner_w - label_w) / 2;
                let label_row = row + 1 + inner_h / 2;
                self.canvas
                    .put_str(col + 1 + pad_left, label_row, label, self.collision)?;
            }
        }

        Ok(())
    }

    fn draw_text(&mut self, text: &Text) -> Result<(), UnidError> {
        self.canvas
            .put_str(text.col, text.row, &text.content, self.collision)
    }

    fn draw_hline(&mut self, hline: &HLine) -> Result<(), UnidError> {
        let ch = hline_char(hline.style);
        for c in 0..hline.length {
            self.canvas
                .put_char(hline.col + c, hline.row, ch, self.collision)?;
        }
        Ok(())
    }

    fn draw_vline(&mut self, vline: &VLine) -> Result<(), UnidError> {
        let ch = vline_char(vline.style);
        for r in 0..vline.length {
            self.canvas
                .put_char(vline.col, vline.row + r, ch, self.collision)?;
        }
        Ok(())
    }

    fn draw_arrow(&mut self, arrow: &Arrow) -> Result<(), UnidError> {
        let (fc, fr, tc, tr) = (arrow.from_col, arrow.from_row, arrow.to_col, arrow.to_row);

        if fr == tr {
            // Horizontal arrow — tip at destination (to_col)
            let (min, max) = if fc < tc { (fc, tc) } else { (tc, fc) };
            let tip = if tc > fc { '→' } else { '←' };
            for c in min..=max {
                self.canvas.put_char(c, fr, '─', self.collision)?;
            }
            self.canvas.put_char(tc, fr, tip, self.collision)?;
        } else if fc == tc {
            // Vertical arrow — tip at destination (to_row)
            let (min, max) = if fr < tr { (fr, tr) } else { (tr, fr) };
            let tip = if tr > fr { '↓' } else { '↑' };
            for r in min..=max {
                self.canvas.put_char(fc, r, '│', self.collision)?;
            }
            self.canvas.put_char(fc, tr, tip, self.collision)?;
        } else {
            // L-shaped arrow: go horizontal first, then vertical
            let v_tip = if tr > fr { '↓' } else { '↑' };

            // Horizontal segment
            let (h_start, h_end) = if tc > fc { (fc, tc) } else { (tc, fc) };
            for c in h_start..=h_end {
                self.canvas.put_char(c, fr, '─', self.collision)?;
            }

            // Corner
            let corner = match (tc > fc, tr > fr) {
                (true, true) => '┐',
                (true, false) => '┘',
                (false, true) => '┌',
                (false, false) => '└',
            };
            self.canvas.put_char(tc, fr, corner, self.collision)?;

            // Vertical segment
            let (v_start, v_end) = if tr > fr { (fr + 1, tr) } else { (tr, fr - 1) };
            for r in v_start..v_end {
                self.canvas.put_char(tc, r, '│', self.collision)?;
            }
            self.canvas.put_char(tc, v_end, v_tip, self.collision)?;
        }

        Ok(())
    }
}

fn border_chars(style: BorderStyle) -> (char, char, char, char, char, char) {
    match style {
        BorderStyle::Light => ('┌', '┐', '└', '┘', '─', '│'),
        BorderStyle::Heavy => ('┏', '┓', '┗', '┛', '━', '┃'),
        BorderStyle::Double => ('╔', '╗', '╚', '╝', '═', '║'),
        BorderStyle::Rounded => ('╭', '╮', '╰', '╯', '─', '│'),
    }
}

fn hline_char(style: LineStyle) -> char {
    match style {
        LineStyle::Light => '─',
        LineStyle::Heavy => '━',
        LineStyle::Double => '═',
        LineStyle::Dash => '╌',
    }
}

fn vline_char(style: LineStyle) -> char {
    match style {
        LineStyle::Light => '│',
        LineStyle::Heavy => '┃',
        LineStyle::Double => '║',
        LineStyle::Dash => '╎',
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::*;
    use pretty_assertions::assert_eq;

    fn render_objects(
        width: usize,
        height: usize,
        objects: &[DrawObject],
        collision: bool,
    ) -> String {
        let canvas = Canvas::new(width, height);
        let mut renderer = Renderer::new(canvas, collision);
        renderer.draw_all(objects).unwrap();
        renderer.render()
    }

    #[test]
    fn render_simple_rect() {
        let rect = Rect::new(0, 0, 4, 1);
        let result = render_objects(6, 3, &[DrawObject::Rect(rect)], false);
        assert_eq!(
            result,
            "\
┌────┐\n\
│    │\n\
└────┘"
        );
    }

    #[test]
    fn render_rect_with_label() {
        let mut rect = Rect::new(0, 0, 8, 1);
        rect.label = Some("Hello".to_string());
        let result = render_objects(10, 3, &[DrawObject::Rect(rect)], false);
        // "Hello" is 5 wide, inner is 8, pad_left = (8-5)/2 = 1
        assert_eq!(
            result,
            "\
┌────────┐\n\
│ Hello  │\n\
└────────┘"
        );
    }

    #[test]
    fn render_rect_with_cjk_label() {
        let mut rect = Rect::new(0, 0, 10, 1);
        rect.label = Some("한글".to_string());
        let result = render_objects(12, 3, &[DrawObject::Rect(rect)], false);
        // "한글" is 4 display cols wide, inner is 10, pad_left = (10-4)/2 = 3
        assert_eq!(
            result,
            "\
┌──────────┐\n\
│   한글   │\n\
└──────────┘"
        );
    }

    #[test]
    fn render_heavy_rect() {
        let mut rect = Rect::new(0, 0, 4, 1);
        rect.style = BorderStyle::Heavy;
        let result = render_objects(6, 3, &[DrawObject::Rect(rect)], false);
        assert_eq!(
            result,
            "\
┏━━━━┓\n\
┃    ┃\n\
┗━━━━┛"
        );
    }

    #[test]
    fn render_double_rect() {
        let mut rect = Rect::new(0, 0, 4, 1);
        rect.style = BorderStyle::Double;
        let result = render_objects(6, 3, &[DrawObject::Rect(rect)], false);
        assert_eq!(
            result,
            "\
╔════╗\n\
║    ║\n\
╚════╝"
        );
    }

    #[test]
    fn render_rounded_rect() {
        let mut rect = Rect::new(0, 0, 4, 1);
        rect.style = BorderStyle::Rounded;
        let result = render_objects(6, 3, &[DrawObject::Rect(rect)], false);
        assert_eq!(
            result,
            "\
╭────╮\n\
│    │\n\
╰────╯"
        );
    }

    #[test]
    fn render_text() {
        let text = Text::new(2, 1, "Hello");
        let result = render_objects(10, 3, &[DrawObject::Text(text)], false);
        assert_eq!(result, "\n  Hello");
    }

    #[test]
    fn render_text_cjk() {
        let text = Text::new(0, 0, "한글ABC");
        let result = render_objects(10, 1, &[DrawObject::Text(text)], false);
        assert_eq!(result, "한글ABC");
    }

    #[test]
    fn render_hline() {
        let hline = HLine::new(1, 0, 5);
        let result = render_objects(7, 1, &[DrawObject::HLine(hline)], false);
        assert_eq!(result, " ─────");
    }

    #[test]
    fn render_hline_heavy() {
        let mut hline = HLine::new(0, 0, 3);
        hline.style = LineStyle::Heavy;
        let result = render_objects(3, 1, &[DrawObject::HLine(hline)], false);
        assert_eq!(result, "━━━");
    }

    #[test]
    fn render_vline() {
        let vline = VLine::new(0, 0, 3);
        let result = render_objects(1, 3, &[DrawObject::VLine(vline)], false);
        assert_eq!(result, "│\n│\n│");
    }

    #[test]
    fn render_horizontal_arrow_right() {
        let arrow = Arrow::new(0, 0, 4, 0);
        let result = render_objects(5, 1, &[DrawObject::Arrow(arrow)], false);
        assert_eq!(result, "────→");
    }

    #[test]
    fn render_horizontal_arrow_left() {
        let arrow = Arrow::new(4, 0, 0, 0);
        let result = render_objects(5, 1, &[DrawObject::Arrow(arrow)], false);
        assert_eq!(result, "←────");
    }

    #[test]
    fn render_vertical_arrow_down() {
        let arrow = Arrow::new(0, 0, 0, 2);
        let result = render_objects(1, 3, &[DrawObject::Arrow(arrow)], false);
        assert_eq!(result, "│\n│\n↓");
    }

    #[test]
    fn render_vertical_arrow_up() {
        let arrow = Arrow::new(0, 2, 0, 0);
        let result = render_objects(1, 3, &[DrawObject::Arrow(arrow)], false);
        assert_eq!(result, "↑\n│\n│");
    }

    #[test]
    fn collision_detected_between_objects() {
        let canvas = Canvas::new(10, 3);
        let mut renderer = Renderer::new(canvas, true);
        let rect = Rect::new(0, 0, 4, 1);
        let text = Text::new(0, 0, "X");
        renderer.draw(&DrawObject::Rect(rect)).unwrap();
        let result = renderer.draw(&DrawObject::Text(text));
        assert!(result.is_err());
    }

    #[test]
    fn collision_off_allows_overlap() {
        let canvas = Canvas::new(10, 3);
        let mut renderer = Renderer::new(canvas, false);
        let rect = Rect::new(0, 0, 4, 1);
        let text = Text::new(0, 0, "X");
        renderer.draw(&DrawObject::Rect(rect)).unwrap();
        renderer.draw(&DrawObject::Text(text)).unwrap();
        // 'X' overwrites the top-left corner
        let output = renderer.render();
        assert!(output.starts_with('X'));
    }

    #[test]
    fn render_multiple_objects() {
        let objects = vec![
            DrawObject::Rect(Rect::new(0, 0, 8, 1)),
            DrawObject::Text(Text::new(15, 1, "World")),
        ];
        let result = render_objects(20, 3, &objects, false);
        assert!(result.contains("┌────────┐"));
        assert!(result.contains("World"));
    }

    #[test]
    fn render_rect_multiline_inner() {
        let mut rect = Rect::new(0, 0, 6, 3);
        rect.label = Some("Hi".to_string());
        let result = render_objects(8, 5, &[DrawObject::Rect(rect)], false);
        // Label "Hi" centered in row 1+3/2 = row 2
        assert_eq!(
            result,
            "\
┌──────┐\n\
│      │\n\
│  Hi  │\n\
│      │\n\
└──────┘"
        );
    }
}
