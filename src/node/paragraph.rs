//! 段落

use super::InlineNode;

#[derive(Debug)]
pub struct ParagraphNode {
    pub contents: Vec<InlineNode>,
}
