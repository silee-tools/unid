use std::process::Command;

fn unid() -> Command {
    Command::new(env!("CARGO_BIN_EXE_unid"))
}

#[test]
fn render_simple_rect_from_inline() {
    let output = unid()
        .args(["render", "--inline", "canvas 6 3, collision off, rect 0 0 4 1"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "┌────┐\n│    │\n└────┘");
}

#[test]
fn render_rect_with_label() {
    let output = unid()
        .args([
            "render",
            "--inline",
            r#"canvas 12 3, collision off, rect 0 0 10 1 "Hello""#,
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Hello"));
    assert!(stdout.contains("┌"));
    assert!(stdout.contains("└"));
}

#[test]
fn render_cjk_label() {
    let output = unid()
        .args([
            "render",
            "--inline",
            r#"canvas 14 3, collision off, rect 0 0 12 1 "한글 테스트""#,
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("한글 테스트"));
}

#[test]
fn render_auto_canvas() {
    let output = unid()
        .args([
            "render",
            "--inline",
            "canvas auto, collision off, rect 0 0 4 1",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "┌────┐\n│    │\n└────┘");
}

#[test]
fn render_collision_on_error() {
    let output = unid()
        .args([
            "render",
            "--inline",
            "canvas 20 5, collision on, rect 0 0 5 1, rect 3 0 5 1",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("collision"));
}

#[test]
fn render_collision_off_allows_overlap() {
    let output = unid()
        .args([
            "render",
            "--inline",
            "canvas 20 5, collision off, rect 0 0 5 1, rect 3 0 5 1",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
}

#[test]
fn render_collision_cli_override() {
    // DSL says collision on, but CLI says off
    let output = unid()
        .args([
            "render",
            "--collision=off",
            "--inline",
            "canvas 20 5, collision on, rect 0 0 5 1, rect 3 0 5 1",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
}

#[test]
fn render_from_file() {
    let dir = std::env::temp_dir().join("unid_test");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("test.unid");
    std::fs::write(
        &path,
        "canvas 8 3\ncollision off\nrect 0 0 6 1\n",
    )
    .unwrap();

    let output = unid()
        .args(["render", "--file", path.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("┌──────┐"));

    std::fs::remove_dir_all(dir).ok();
}

#[test]
fn render_from_stdin() {
    let output = unid()
        .arg("render")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            child
                .stdin
                .take()
                .unwrap()
                .write_all(b"canvas 6 3\ncollision off\nrect 0 0 4 1\n")
                .unwrap();
            child.wait_with_output()
        })
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("┌────┐"));
}

#[test]
fn list_subcommand() {
    let output = unid()
        .args([
            "list",
            "--inline",
            r#"canvas 30 5, collision on, rect 0 0 8 1 "Box", text 15 1 "Hi""#,
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Canvas: 30x5"));
    assert!(stdout.contains("Collision: on"));
    assert!(stdout.contains("Objects: 2"));
    assert!(stdout.contains("rect"));
    assert!(stdout.contains("text"));
}

#[test]
fn list_auto_canvas() {
    let output = unid()
        .args([
            "list",
            "--inline",
            "canvas auto, collision off, rect 0 0 4 1",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("(auto)"));
}

#[test]
fn guide_subcommand() {
    let output = unid().arg("guide").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("USAGE:"));
    assert!(stdout.contains("DSL SYNTAX:"));
    assert!(stdout.contains("BORDER STYLES:"));
    assert!(stdout.contains("collision on"));
    assert!(stdout.contains("collision off"));
}

#[test]
fn render_multiple_styles() {
    let input = "\
canvas 30 12, collision off,\
rect 0 0 6 1 style=light,\
rect 0 3 6 1 style=heavy,\
rect 0 6 6 1 style=double,\
rect 0 9 6 1 style=rounded";
    let output = unid()
        .args(["render", "--inline", input])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains('┌')); // light
    assert!(stdout.contains('┏')); // heavy
    assert!(stdout.contains('╔')); // double
    assert!(stdout.contains('╭')); // rounded
}

#[test]
fn render_arrows() {
    let output = unid()
        .args([
            "render",
            "--inline",
            "canvas 10 5, collision off, arrow 0 0 9 0, arrow 0 2 0 4",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains('→'));
    assert!(stdout.contains('↓'));
}

#[test]
fn render_lines() {
    let output = unid()
        .args([
            "render",
            "--inline",
            "canvas 10 5, collision off, hline 0 0 5, vline 0 1 4",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains('─'));
    assert!(stdout.contains('│'));
}

#[test]
fn render_cjk_mixed_diagram() {
    let input = r#"canvas 30 5, collision off, rect 0 0 12 1 "서버", rect 18 0 8 1 "DB", arrow 14 1 18 1"#;
    let output = unid()
        .args(["render", "--inline", input])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("서버"));
    assert!(stdout.contains("DB"));
    assert!(stdout.contains('→'));
}

#[test]
fn error_missing_canvas() {
    let output = unid()
        .args(["render", "--inline", "collision on, rect 0 0 4 1"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("canvas"));
}

#[test]
fn error_missing_collision() {
    let output = unid()
        .args(["render", "--inline", "canvas 10 5, rect 0 0 4 1"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("collision"));
}

#[test]
fn error_parse_error() {
    let output = unid()
        .args(["render", "--inline", "canvas 10 5, collision on, badcmd 1 2"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("unknown command"));
}
