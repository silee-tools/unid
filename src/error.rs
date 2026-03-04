use thiserror::Error;

#[derive(Debug, Error)]
pub enum UnidError {
    #[error("canvas not defined")]
    NoCanvas,

    #[error("collision not defined")]
    NoCollision,

    #[error("position ({col}, {row}) is out of canvas bounds ({canvas_width}x{canvas_height})")]
    OutOfBounds {
        col: usize,
        row: usize,
        canvas_width: usize,
        canvas_height: usize,
    },

    #[error("collision at ({col}, {row}) between '{existing}' and '{incoming}'")]
    Collision {
        col: usize,
        row: usize,
        existing: String,
        incoming: String,
    },

    #[error("parse error at line {line}: {message}")]
    Parse { line: usize, message: String },

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
