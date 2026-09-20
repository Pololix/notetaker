#[derive(Debug, Clone)]
pub struct Attachment {
    row_start: u32,
    col_start: u32,
    row_end: u32,
    col_end: u32,

    ty: AttachmentType,
}

#[derive(Debug, Clone)]
pub enum AttachmentType {
    Filler,
}
