//! 強調

use super::PlainTextNode;

#[derive(Debug)]
pub enum EmphNode {
    Italic(ItalicNode),
    Bold(BoldNode),
}

#[derive(Debug)]
pub struct ItalicNode {
    pub contents: PlainTextNode,
}

#[derive(Debug)]
pub struct BoldNode {
    pub contents: PlainTextNode,
}
