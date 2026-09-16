use crate::document::grid::Grid;
use ropey::Rope;

#[derive(Debug, Default)]
pub struct Document {
    rope: Rope,
    grid: Grid,
}
