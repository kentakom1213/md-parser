//! 強調を表すノード

#[derive(Debug)]
pub enum EmphNode {
    Italic(ItalicNode),
    Bold(BoldNode),
}

#[derive(Debug)]
pub struct ItalicNode {
    pub contents: String,
}

#[derive(Debug)]
pub struct BoldNode {
    pub contents: String,
}
