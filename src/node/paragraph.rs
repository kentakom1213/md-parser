//! 段落を表すノード

use super::InlineNode;

pub struct ParagraphNode {
    pub contents: Vec<InlineNode>,
}
