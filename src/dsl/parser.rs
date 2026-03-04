use crate::dsl::command::{CanvasSize, DslCommand};
use crate::error::UnidError;
use crate::object::{Arrow, BorderStyle, DrawObject, HLine, LineStyle, Rect, Text, VLine};

/// Parses DSL text into a list of commands.
/// Supports both newline-separated and comma-separated (inline) formats.
pub fn parse(input: &str) -> Result<Vec<DslCommand>, UnidError> {
    let lines = split_lines(input);
    let mut commands = Vec::new();

    for (line_num, line) in lines.iter().enumerate() {
        let line_num = line_num + 1; // 1-based
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let tokens = tokenize(trimmed);
        if tokens.is_empty() {
            continue;
        }

        let cmd = parse_command(&tokens, line_num)?;
        commands.push(cmd);
    }

    Ok(commands)
}

/// Splits input into lines. Commas act as line separators (for inline mode).
fn split_lines(input: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in input.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
                current.push(ch);
            }
            ',' if !in_quotes => {
                lines.push(current.clone());
                current.clear();
            }
            '\n' if !in_quotes => {
                lines.push(current.clone());
                current.clear();
            }
            _ => {
                current.push(ch);
            }
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

/// Tokenizes a line into whitespace-separated tokens, respecting quoted strings.
fn tokenize(line: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in line.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
                // Don't include the quotes in the token
            }
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(ch);
            }
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

fn parse_command(tokens: &[String], line: usize) -> Result<DslCommand, UnidError> {
    let keyword = tokens[0].to_lowercase();
    match keyword.as_str() {
        "canvas" => parse_canvas(tokens, line),
        "collision" => parse_collision(tokens, line),
        "rect" => parse_rect(tokens, line),
        "text" => parse_text(tokens, line),
        "hline" => parse_hline(tokens, line),
        "vline" => parse_vline(tokens, line),
        "arrow" => parse_arrow(tokens, line),
        _ => Err(UnidError::Parse {
            line,
            message: format!("unknown command '{}'", tokens[0]),
        }),
    }
}

fn parse_canvas(tokens: &[String], line: usize) -> Result<DslCommand, UnidError> {
    if tokens.len() < 2 {
        return Err(UnidError::Parse {
            line,
            message: "canvas requires size arguments (e.g., 'canvas 40 10' or 'canvas auto')"
                .to_string(),
        });
    }

    if tokens[1].to_lowercase() == "auto" {
        return Ok(DslCommand::Canvas {
            width: CanvasSize::Auto,
            height: CanvasSize::Auto,
        });
    }

    if tokens.len() < 3 {
        return Err(UnidError::Parse {
            line,
            message: "canvas requires width and height (e.g., 'canvas 40 10')".to_string(),
        });
    }

    let width = parse_usize(&tokens[1], "canvas width", line)?;
    let height = parse_usize(&tokens[2], "canvas height", line)?;

    Ok(DslCommand::Canvas {
        width: CanvasSize::Fixed(width),
        height: CanvasSize::Fixed(height),
    })
}

fn parse_collision(tokens: &[String], line: usize) -> Result<DslCommand, UnidError> {
    if tokens.len() < 2 {
        return Err(UnidError::Parse {
            line,
            message: "collision requires 'on' or 'off'".to_string(),
        });
    }

    match tokens[1].to_lowercase().as_str() {
        "on" => Ok(DslCommand::Collision(true)),
        "off" => Ok(DslCommand::Collision(false)),
        _ => Err(UnidError::Parse {
            line,
            message: format!("collision must be 'on' or 'off', got '{}'", tokens[1]),
        }),
    }
}

fn parse_rect(tokens: &[String], line: usize) -> Result<DslCommand, UnidError> {
    // rect <col> <row> <width> <height> ["label"] [style=light|heavy|double|rounded]
    if tokens.len() < 5 {
        return Err(UnidError::Parse {
            line,
            message: "rect requires col, row, width, height".to_string(),
        });
    }

    let col = parse_usize(&tokens[1], "col", line)?;
    let row = parse_usize(&tokens[2], "row", line)?;
    let width = parse_usize(&tokens[3], "width", line)?;
    let height = parse_usize(&tokens[4], "height", line)?;

    let mut rect = Rect::new(col, row, width, height);

    // Parse optional label and style
    for token in &tokens[5..] {
        if let Some(style_str) = token.strip_prefix("style=") {
            rect.style = parse_border_style(style_str, line)?;
        } else {
            rect.label = Some(token.clone());
        }
    }

    Ok(DslCommand::Object(DrawObject::Rect(rect)))
}

fn parse_text(tokens: &[String], line: usize) -> Result<DslCommand, UnidError> {
    // text <col> <row> "content"
    if tokens.len() < 4 {
        return Err(UnidError::Parse {
            line,
            message: "text requires col, row, content".to_string(),
        });
    }

    let col = parse_usize(&tokens[1], "col", line)?;
    let row = parse_usize(&tokens[2], "row", line)?;
    let content = tokens[3..].join(" ");

    Ok(DslCommand::Object(DrawObject::Text(Text::new(
        col, row, content,
    ))))
}

fn parse_hline(tokens: &[String], line: usize) -> Result<DslCommand, UnidError> {
    // hline <col> <row> <length> [style=...]
    if tokens.len() < 4 {
        return Err(UnidError::Parse {
            line,
            message: "hline requires col, row, length".to_string(),
        });
    }

    let col = parse_usize(&tokens[1], "col", line)?;
    let row = parse_usize(&tokens[2], "row", line)?;
    let length = parse_usize(&tokens[3], "length", line)?;

    let mut hline = HLine::new(col, row, length);

    if let Some(token) = tokens.get(4)
        && let Some(style_str) = token.strip_prefix("style=")
    {
        hline.style = parse_line_style(style_str, line)?;
    }

    Ok(DslCommand::Object(DrawObject::HLine(hline)))
}

fn parse_vline(tokens: &[String], line: usize) -> Result<DslCommand, UnidError> {
    // vline <col> <row> <length> [style=...]
    if tokens.len() < 4 {
        return Err(UnidError::Parse {
            line,
            message: "vline requires col, row, length".to_string(),
        });
    }

    let col = parse_usize(&tokens[1], "col", line)?;
    let row = parse_usize(&tokens[2], "row", line)?;
    let length = parse_usize(&tokens[3], "length", line)?;

    let mut vline = VLine::new(col, row, length);

    if let Some(token) = tokens.get(4)
        && let Some(style_str) = token.strip_prefix("style=")
    {
        vline.style = parse_line_style(style_str, line)?;
    }

    Ok(DslCommand::Object(DrawObject::VLine(vline)))
}

fn parse_arrow(tokens: &[String], line: usize) -> Result<DslCommand, UnidError> {
    // arrow <from_col> <from_row> <to_col> <to_row>
    if tokens.len() < 5 {
        return Err(UnidError::Parse {
            line,
            message: "arrow requires from_col, from_row, to_col, to_row".to_string(),
        });
    }

    let from_col = parse_usize(&tokens[1], "from_col", line)?;
    let from_row = parse_usize(&tokens[2], "from_row", line)?;
    let to_col = parse_usize(&tokens[3], "to_col", line)?;
    let to_row = parse_usize(&tokens[4], "to_row", line)?;

    Ok(DslCommand::Object(DrawObject::Arrow(Arrow::new(
        from_col, from_row, to_col, to_row,
    ))))
}

fn parse_border_style(s: &str, line: usize) -> Result<BorderStyle, UnidError> {
    match s.to_lowercase().as_str() {
        "light" => Ok(BorderStyle::Light),
        "heavy" => Ok(BorderStyle::Heavy),
        "double" => Ok(BorderStyle::Double),
        "rounded" => Ok(BorderStyle::Rounded),
        _ => Err(UnidError::Parse {
            line,
            message: format!(
                "unknown border style '{}' (expected light, heavy, double, rounded)",
                s
            ),
        }),
    }
}

fn parse_line_style(s: &str, line: usize) -> Result<LineStyle, UnidError> {
    match s.to_lowercase().as_str() {
        "light" => Ok(LineStyle::Light),
        "heavy" => Ok(LineStyle::Heavy),
        "double" => Ok(LineStyle::Double),
        "dash" => Ok(LineStyle::Dash),
        _ => Err(UnidError::Parse {
            line,
            message: format!(
                "unknown line style '{}' (expected light, heavy, double, dash)",
                s
            ),
        }),
    }
}

fn parse_usize(s: &str, name: &str, line: usize) -> Result<usize, UnidError> {
    s.parse().map_err(|_| UnidError::Parse {
        line,
        message: format!("invalid {} '{}' (expected a non-negative integer)", name, s),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::command::CanvasSize;

    #[test]
    fn parse_canvas_fixed() {
        let cmds = parse("canvas 40 10").unwrap();
        assert_eq!(cmds.len(), 1);
        match &cmds[0] {
            DslCommand::Canvas { width, height } => {
                assert_eq!(*width, CanvasSize::Fixed(40));
                assert_eq!(*height, CanvasSize::Fixed(10));
            }
            _ => panic!("expected Canvas"),
        }
    }

    #[test]
    fn parse_canvas_auto() {
        let cmds = parse("canvas auto").unwrap();
        match &cmds[0] {
            DslCommand::Canvas { width, height } => {
                assert_eq!(*width, CanvasSize::Auto);
                assert_eq!(*height, CanvasSize::Auto);
            }
            _ => panic!("expected Canvas"),
        }
    }

    #[test]
    fn parse_collision_on_off() {
        let on = parse("collision on").unwrap();
        let off = parse("collision off").unwrap();
        match &on[0] {
            DslCommand::Collision(v) => assert!(*v),
            _ => panic!("expected Collision"),
        }
        match &off[0] {
            DslCommand::Collision(v) => assert!(!(*v)),
            _ => panic!("expected Collision"),
        }
    }

    #[test]
    fn parse_rect_basic() {
        let cmds = parse("rect 0 0 10 3").unwrap();
        match &cmds[0] {
            DslCommand::Object(DrawObject::Rect(r)) => {
                assert_eq!((r.col, r.row, r.width, r.height), (0, 0, 10, 3));
                assert!(r.label.is_none());
                assert_eq!(r.style, BorderStyle::Light);
            }
            _ => panic!("expected Rect"),
        }
    }

    #[test]
    fn parse_rect_with_label_and_style() {
        let cmds = parse(r#"rect 0 0 10 3 "Hello World" style=rounded"#).unwrap();
        match &cmds[0] {
            DslCommand::Object(DrawObject::Rect(r)) => {
                assert_eq!(r.label.as_deref(), Some("Hello World"));
                assert_eq!(r.style, BorderStyle::Rounded);
            }
            _ => panic!("expected Rect"),
        }
    }

    #[test]
    fn parse_text() {
        let cmds = parse(r#"text 5 3 "Hello 한글""#).unwrap();
        match &cmds[0] {
            DslCommand::Object(DrawObject::Text(t)) => {
                assert_eq!(t.col, 5);
                assert_eq!(t.row, 3);
                assert_eq!(t.content, "Hello 한글");
            }
            _ => panic!("expected Text"),
        }
    }

    #[test]
    fn parse_hline() {
        let cmds = parse("hline 0 5 20 style=heavy").unwrap();
        match &cmds[0] {
            DslCommand::Object(DrawObject::HLine(h)) => {
                assert_eq!((h.col, h.row, h.length), (0, 5, 20));
                assert_eq!(h.style, LineStyle::Heavy);
            }
            _ => panic!("expected HLine"),
        }
    }

    #[test]
    fn parse_vline() {
        let cmds = parse("vline 10 0 5").unwrap();
        match &cmds[0] {
            DslCommand::Object(DrawObject::VLine(v)) => {
                assert_eq!((v.col, v.row, v.length), (10, 0, 5));
                assert_eq!(v.style, LineStyle::Light);
            }
            _ => panic!("expected VLine"),
        }
    }

    #[test]
    fn parse_arrow() {
        let cmds = parse("arrow 0 0 10 5").unwrap();
        match &cmds[0] {
            DslCommand::Object(DrawObject::Arrow(a)) => {
                assert_eq!((a.from_col, a.from_row, a.to_col, a.to_row), (0, 0, 10, 5));
            }
            _ => panic!("expected Arrow"),
        }
    }

    #[test]
    fn parse_comments_and_blank_lines() {
        let input = "\
# This is a comment
canvas 20 5

collision on

# Another comment
rect 0 0 5 2
";
        let cmds = parse(input).unwrap();
        assert_eq!(cmds.len(), 3);
    }

    #[test]
    fn parse_inline_comma_separated() {
        let input = r#"canvas 20 5, collision off, rect 0 0 5 2 "Hi""#;
        let cmds = parse(input).unwrap();
        assert_eq!(cmds.len(), 3);
    }

    #[test]
    fn parse_case_insensitive() {
        let cmds = parse("CANVAS 20 5").unwrap();
        assert!(matches!(cmds[0], DslCommand::Canvas { .. }));

        let cmds = parse("Rect 0 0 5 2").unwrap();
        assert!(matches!(cmds[0], DslCommand::Object(DrawObject::Rect(_))));
    }

    #[test]
    fn parse_error_unknown_command() {
        let result = parse("unknown 1 2 3");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("unknown command"));
        assert!(err.contains("line 1"));
    }

    #[test]
    fn parse_error_invalid_number() {
        let result = parse("rect abc 0 5 2");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("invalid col"));
    }

    #[test]
    fn parse_error_missing_args() {
        let result = parse("rect 0 0");
        assert!(result.is_err());
    }

    #[test]
    fn parse_quoted_string_with_comma() {
        let input = r#"text 0 0 "Hello, World""#;
        let cmds = parse(input).unwrap();
        match &cmds[0] {
            DslCommand::Object(DrawObject::Text(t)) => {
                assert_eq!(t.content, "Hello, World");
            }
            _ => panic!("expected Text"),
        }
    }
}
