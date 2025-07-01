//! 強調を表すノード

pub enum EmphNode {
    Italic(ItalicNode),
    Bold(BoldNode),
}

pub struct ItalicNode {
    pub contents: String,
}

pub struct BoldNode {
    pub contents: String,
}
