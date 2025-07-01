//! 段落を表すノード

use super::InlineNode;

#[derive(Debug)]
pub struct ParagraphNode {
    pub contents: Vec<InlineNode>,
}
