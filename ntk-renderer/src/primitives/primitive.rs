#[derive(Debug, Clone, Copy)]
pub enum Primitive {
    Quad(Quad),
    Line(Line),
    Dot(Dot),
}

#[derive(Debug, Clone, Copy)]
pub struct Quad {}

#[derive(Debug, Clone, Copy)]
pub struct Line {}

#[derive(Debug, Clone, Copy)]
pub struct Dot {}
