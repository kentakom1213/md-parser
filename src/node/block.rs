//! ブロックを表すノード

use super::{CodeBlockNode, HeadingNode, MathBlockNode, ParagraphNode};

pub enum BlockNode {
    Breaking,
    Heading(HeadingNode),
    CodeBlock(CodeBlockNode),
    MathBlockNode(MathBlockNode),
    ParagraphNode(ParagraphNode),
}
