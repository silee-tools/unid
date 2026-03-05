use std::io::Write;
use std::process::{Command, Stdio};

fn unid() -> Command {
    Command::new(env!("CARGO_BIN_EXE_unid"))
}

/// Pipe DSL input to unid via stdin and return (stdout, stderr, success).
fn run_stdin(input: &str) -> (String, String, bool) {
    let mut child = unid()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    (
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
        output.status.success(),
    )
}

/// Pipe DSL input to a subcommand (list, lint).
fn run_subcmd(subcmd: &str, input: &str) -> (String, String, bool) {
    let mut child = unid()
        .arg(subcmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    (
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
        output.status.success(),
    )
}

/// Golden file test: load DSL from fixture, render, compare with expected output.
macro_rules! golden_test {
    ($name:ident) => {
        #[test]
        fn $name() {
            let dsl = include_str!(concat!("fixtures/", stringify!($name), ".dsl"));
            let expected = include_str!(concat!("fixtures/", stringify!($name), ".golden"));
            let (stdout, stderr, ok) = run_stdin(dsl);
            assert!(ok, "render failed for {}: {stderr}", stringify!($name));
            println!("{stdout}");
            pretty_assertions::assert_eq!(stdout, expected);
        }
    };
}

// ─── Render (golden file tests) ─────────────────────────────────────

golden_test!(render_simple_rect);
golden_test!(render_rect_with_content);
golden_test!(render_cjk_content);
golden_test!(render_auto_canvas);
golden_test!(render_multiple_styles);
golden_test!(render_anchor_arrow_horizontal);
golden_test!(render_anchor_arrow_vertical);
golden_test!(render_anchor_arrow_l_shape);
golden_test!(render_anchor_arrow_u_shape);
golden_test!(render_lines);
golden_test!(render_cjk_mixed_diagram);
golden_test!(render_text_object);
golden_test!(render_complex_architecture_diagram);

// ─── Collision ──────────────────────────────────────────────────────

golden_test!(collision_off_allows_overlap);

#[test]
fn collision_on_error() {
    let (_, stderr, ok) = run_stdin(
        "canvas 20 5\n\
         collision on\n\
         box 0 0 5 1\n\
         box 3 0 5 1",
    );
    assert!(!ok);
    assert!(stderr.contains("collision"));
}

#[test]
fn collision_error_format() {
    let (_, stderr, ok) = run_stdin(
        "canvas 20 5\n\
         collision on\n\
         box 0 0 5 1\n\
         box 3 0 5 1",
    );
    assert!(!ok);
    assert!(stderr.contains("object #2"));
    assert!(stderr.contains("object #1"));
    assert!(stderr.contains("overlaps"));
    assert!(stderr.contains("size"));
}

// ─── Content Overflow ───────────────────────────────────────────────

golden_test!(overflow_ellipsis);
golden_test!(overflow_hidden);

#[test]
fn overflow_error() {
    let (_, stderr, ok) = run_stdin(
        "canvas 10 3\n\
         collision off\n\
         box 0 0 3 1 overflow=error c=VeryLong",
    );
    assert!(!ok);
    assert!(stderr.contains("overflow"));
}

// ─── Content Alignment ─────────────────────────────────────────────

golden_test!(align_center);
golden_test!(align_right);

// ─── Canvas Border ──────────────────────────────────────────────────

golden_test!(canvas_border_rounded);

// ─── List ───────────────────────────────────────────────────────────

#[test]
fn list_subcommand() {
    let (stdout, _, ok) = run_subcmd(
        "list",
        "canvas 30 5\n\
         collision on\n\
         box 0 0 8 1 c=Box\n\
         text 15 1 c=Hi",
    );
    assert!(ok);
    assert!(stdout.contains("Canvas: 30x5"));
    assert!(stdout.contains("Collision: on"));
    assert!(stdout.contains("Objects: 2"));
    assert!(stdout.contains("box"));
    assert!(stdout.contains("text"));
}

#[test]
fn list_auto_canvas() {
    let (stdout, _, ok) = run_subcmd(
        "list",
        "canvas auto\n\
         collision off\n\
         box 0 0 4 1",
    );
    assert!(ok);
    assert!(stdout.contains("(auto)"));
}

// ─── Lint ───────────────────────────────────────────────────────────

#[test]
fn lint_ok() {
    let (stdout, _, ok) = run_subcmd(
        "lint",
        "canvas 10 3\n\
         collision off\n\
         box 0 0 4 1",
    );
    assert!(ok);
    assert!(stdout.contains("OK"));
}

#[test]
fn lint_collision_error() {
    let (stdout, _, ok) = run_subcmd(
        "lint",
        "canvas 10 5\n\
         collision on\n\
         box 0 0 5 1\n\
         box 3 0 5 1",
    );
    assert!(!ok);
    assert!(stdout.contains("Errors:"));
    assert!(stdout.contains("collision"));
}

// ─── Guide ──────────────────────────────────────────────────────────

#[test]
fn guide_subcommand() {
    let output = unid().arg("guide").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("USAGE:"));
    assert!(stdout.contains("DSL SYNTAX:"));
    assert!(stdout.contains("BORDER STYLES"));
}

// ─── Error cases ────────────────────────────────────────────────────

#[test]
fn error_missing_canvas() {
    let (_, stderr, ok) = run_stdin(
        "collision on\n\
         box 0 0 4 1",
    );
    assert!(!ok);
    assert!(stderr.contains("canvas"));
}

#[test]
fn error_missing_collision() {
    let (_, stderr, ok) = run_stdin(
        "canvas 10 5\n\
         box 0 0 4 1",
    );
    assert!(!ok);
    assert!(stderr.contains("collision"));
}

#[test]
fn error_parse_error() {
    let (_, stderr, ok) = run_stdin(
        "canvas 10 5\n\
         collision on\n\
         badcmd 1 2",
    );
    assert!(!ok);
    assert!(stderr.contains("unknown command"));
}

#[test]
fn error_unknown_arrow_id() {
    let (_, stderr, ok) = run_stdin(
        "canvas 20 5\n\
         collision off\n\
         box 0 0 4 1 id=a\n\
         arrow a.r nonexistent.l",
    );
    assert!(!ok);
    assert!(stderr.contains("unknown object id"));
}

#[test]
fn error_invalid_arrow_anchor() {
    let (_, stderr, ok) = run_stdin(
        "canvas 20 5\n\
         collision off\n\
         arrow noid db.top",
    );
    assert!(!ok);
    assert!(stderr.contains("invalid anchor"));
}

#[test]
fn duplicate_id_error() {
    let (_, stderr, ok) = run_stdin(
        "canvas 20 5\n\
         collision off\n\
         box 0 0 4 1 id=a\n\
         box 10 0 4 1 id=a",
    );
    assert!(!ok);
    assert!(stderr.contains("duplicate"));
}

#[test]
fn arrow_invalid_arrowhead_rejected() {
    let (_, stderr, ok) = run_stdin(
        "canvas 30 3\n\
         collision off\n\
         box 0 0 4 1 id=a c=A\n\
         box 20 0 4 1 id=b c=B\n\
         arrow a.r b.l head=◆",
    );
    assert!(!ok);
    assert!(stderr.contains("invalid arrowhead"));
}

#[test]
fn rect_legend_lr_error() {
    let (_, stderr, ok) = run_stdin(
        "canvas 20 5\n\
         collision off\n\
         box 0 0 10 1 legend-pos=l lg=Bad",
    );
    assert!(!ok);
    assert!(stderr.contains("legend-pos only supports top"));
}

// ─── Comments and blank lines ───────────────────────────────────────

golden_test!(comments_and_blank_lines);

// ─── Backward compatibility ─────────────────────────────────────────

golden_test!(rect_alias_for_box);

// ─── Shorthand options ──────────────────────────────────────────────

golden_test!(shorthand_style);
golden_test!(content_with_newline_escape);
golden_test!(multiline_rect_vertical_center);
golden_test!(multiline_text_object);

// ─── Legend ──────────────────────────────────────────────────────────

golden_test!(rect_legend_top);
golden_test!(rect_legend_bottom);
golden_test!(rect_content_and_legend);
golden_test!(hline_legend_top);
golden_test!(vline_legend_right);
golden_test!(hline_with_id);

// ─── Arrow from non-rect objects ────────────────────────────────────

golden_test!(arrow_from_hline);
golden_test!(arrow_from_text);
golden_test!(arrow_from_vline);

// ─── Rect ID ────────────────────────────────────────────────────────

golden_test!(rect_with_id);

// ─── Arrowhead + Bidirectional ──────────────────────────────────────

golden_test!(arrow_custom_head);
golden_test!(arrow_global_arrowhead);
golden_test!(arrow_per_arrow_overrides_global);
golden_test!(arrow_bidirectional);
golden_test!(arrow_bidirectional_with_custom_head);
golden_test!(arrow_head_resolves_direction_vertical);

// ─── Arrow Legend ───────────────────────────────────────────────────

golden_test!(arrow_legend_horizontal);
golden_test!(arrow_legend_with_pos);
golden_test!(arrow_legend_vertical);

// ─── Text overwrite ─────────────────────────────────────────────────

golden_test!(text_overwrites_structure);

// ─── Self-loop ──────────────────────────────────────────────────────

golden_test!(self_loop_right_to_top);
golden_test!(self_loop_bottom_to_left);
