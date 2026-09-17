use crate::document::cell_map::CellMap;
use ropey::Rope;

const DOCUMENT_SIZE: f32 = 50.0;
const MARGIN: f32 = 10.0;

#[derive(Debug, Default)]
pub struct Document {
    text: Rope,
    grid: CellMap,
}
