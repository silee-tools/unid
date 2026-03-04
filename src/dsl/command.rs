use crate::object::DrawObject;

/// Represents a parsed DSL command.
#[derive(Debug)]
pub enum DslCommand {
    /// Canvas size declaration.
    Canvas { width: CanvasSize, height: CanvasSize },
    /// Collision mode declaration.
    Collision(bool),
    /// A drawable object.
    Object(DrawObject),
}

/// Canvas dimension: explicit size or auto.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasSize {
    Fixed(usize),
    Auto,
}
