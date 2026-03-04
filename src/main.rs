use std::io::Read;
use std::process;

use clap::Parser;
use unicode_diagram::canvas::Canvas;
use unicode_diagram::cli::{Cli, CollisionMode, Commands};
use unicode_diagram::dsl::command::{CanvasSize, DslCommand};
use unicode_diagram::dsl::parse;
use unicode_diagram::error::UnidError;
use unicode_diagram::object::DrawObject;
use unicode_diagram::renderer::Renderer;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Render {
            file,
            inline,
            collision,
        } => run_render(file, inline, collision),
        Commands::List { file, inline } => run_list(file, inline),
        Commands::Guide => {
            print_help();
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        process::exit(1);
    }
}

fn read_input(file: Option<String>, inline: Option<String>) -> Result<String, UnidError> {
    if let Some(inline_str) = inline {
        return Ok(inline_str);
    }
    if let Some(path) = file {
        return std::fs::read_to_string(&path)
            .map_err(|e| UnidError::Io(std::io::Error::new(e.kind(), format!("{path}: {e}"))));
    }
    // Read from stdin
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}

fn process_commands(
    commands: Vec<DslCommand>,
    collision_override: Option<CollisionMode>,
) -> Result<(CanvasSize, CanvasSize, bool, Vec<DrawObject>), UnidError> {
    let mut canvas_width = None;
    let mut canvas_height = None;
    let mut collision = None;
    let mut objects = Vec::new();

    for cmd in commands {
        match cmd {
            DslCommand::Canvas { width, height } => {
                canvas_width = Some(width);
                canvas_height = Some(height);
            }
            DslCommand::Collision(v) => {
                collision = Some(v);
            }
            DslCommand::Object(obj) => {
                objects.push(obj);
            }
        }
    }

    let cw = canvas_width.ok_or(UnidError::NoCanvas)?;
    let ch = canvas_height.ok_or(UnidError::NoCanvas)?;
    let coll_dsl = collision.ok_or(UnidError::NoCollision)?;

    let coll = match collision_override {
        Some(CollisionMode::On) => true,
        Some(CollisionMode::Off) => false,
        None => coll_dsl,
    };

    Ok((cw, ch, coll, objects))
}

fn compute_canvas_size(
    width: CanvasSize,
    height: CanvasSize,
    objects: &[DrawObject],
) -> (usize, usize) {
    match (width, height) {
        (CanvasSize::Fixed(w), CanvasSize::Fixed(h)) => (w, h),
        _ => {
            let (mut max_w, mut max_h) = (1, 1);
            for obj in objects {
                let (bw, bh) = obj.bounds();
                max_w = max_w.max(bw);
                max_h = max_h.max(bh);
            }
            let w = if let CanvasSize::Fixed(fw) = width {
                fw
            } else {
                max_w
            };
            let h = if let CanvasSize::Fixed(fh) = height {
                fh
            } else {
                max_h
            };
            (w, h)
        }
    }
}

fn run_render(
    file: Option<String>,
    inline: Option<String>,
    collision_override: Option<CollisionMode>,
) -> Result<(), UnidError> {
    let input = read_input(file, inline)?;
    let commands = parse(&input)?;
    let (cw, ch, collision, objects) = process_commands(commands, collision_override)?;
    let (width, height) = compute_canvas_size(cw, ch, &objects);

    let canvas = Canvas::new(width, height);
    let mut renderer = Renderer::new(canvas, collision);
    renderer.draw_all(&objects)?;
    println!("{}", renderer.render());
    Ok(())
}

fn run_list(file: Option<String>, inline: Option<String>) -> Result<(), UnidError> {
    let input = read_input(file, inline)?;
    let commands = parse(&input)?;
    let (cw, ch, collision, mut objects) = process_commands(commands, None)?;
    let (width, height) = compute_canvas_size(cw, ch, &objects);

    let auto_label = match (cw, ch) {
        (CanvasSize::Auto, CanvasSize::Auto) => " (auto)",
        _ => "",
    };

    println!("Canvas: {}x{}{}", width, height, auto_label);
    println!("Collision: {}", if collision { "on" } else { "off" });
    println!("Objects: {}", objects.len());

    // Sort by position: row first, then col
    objects.sort_by(|a, b| {
        let (ac, ar) = a.position();
        let (bc, br) = b.position();
        (ar, ac).cmp(&(br, bc))
    });

    for (i, obj) in objects.iter().enumerate() {
        println!("  {}. {}", i + 1, obj.summary());
    }

    Ok(())
}

fn print_help() {
    print!(
        r#"unid - Unicode Diagram Renderer

USAGE:
  unid render --file <path>              Render from file
  unid render --inline "<dsl>"           Render from inline string
  cat file.unid | unid render            Render from stdin
  unid render --collision=off --file f   Override collision mode
  unid list --file <path>                List objects in diagram
  unid help                              Show this guide

DSL SYNTAX:
  Lines starting with # are comments. Blank lines are ignored.
  Commands are case-insensitive. Inline mode uses commas as separators.

  HEADER (required, must appear before objects):
    canvas <width> <height>              Fixed canvas size
    canvas auto                          Auto-size from objects
    collision on|off                     Collision detection mode

  OBJECTS:
    rect <col> <row> <w> <h> ["label"] [style=light|heavy|double|rounded]
    text <col> <row> "content"
    hline <col> <row> <length> [style=light|heavy|double|dash]
    vline <col> <row> <length> [style=light|heavy|double|dash]
    arrow <from_col> <from_row> <to_col> <to_row>

  Coordinates are display-column based (CJK characters occupy 2 columns).
  Rect width/height are inner dimensions (borders add +2 each).

BORDER STYLES:
  light (default):  ┌─┐ │ └─┘
  heavy:            ┏━┓ ┃ ┗━┛
  double:           ╔═╗ ║ ╚═╝
  rounded:          ╭─╮ │ ╰─╯

LINE STYLES:
  light (default):  ─ │
  heavy:            ━ ┃
  double:           ═ ║
  dash:             ╌ ╎

ARROWS:
  Horizontal: ──→  ←──
  Vertical:   │↓   ↑│
  L-shaped:   ──┐  (horizontal first, then vertical)
                ↓

EXAMPLE (collision off):
  canvas 30 7
  collision off
  rect 0 0 10 3 "Server" style=rounded
  rect 16 0 10 3 "Client"
  arrow 12 2 16 2
  text 0 6 "System Architecture"

  Output:
  ╭──────────╮    ┌──────────┐
  │          │    │          │
  │  Server  │──→ │  Client  │
  │          │    │          │
  ╰──────────╯    └──────────┘

  System Architecture

EXAMPLE (collision on):
  canvas 30 5
  collision on
  rect 0 0 8 3
  rect 5 1 8 3

  Output:
  error: collision at (5, 1) between '│' and '┌'

CJK EXAMPLE:
  canvas 20 3
  collision off
  rect 0 0 10 1 "한글 테스트"

  Output:
  ┌──────────┐
  │한글 테스트│
  └──────────┘

NOTES:
  - --collision CLI flag overrides DSL collision declaration
  - Canvas auto computes minimum size from all object bounds
  - CJK characters (한글, 漢字, かな) take 2 display columns
  - Label is centered within rect inner width
"#
    );
}
