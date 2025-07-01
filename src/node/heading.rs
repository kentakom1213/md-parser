//! 見出しを表すノード

#[derive(Debug)]
pub struct HeadingNode {
    pub level: u8,
    pub contents: String,
}
