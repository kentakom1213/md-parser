//! ブロック

use super::{CodeBlockNode, HeadingNode, MathBlockNode, ParagraphNode};

#[derive(Debug)]
pub enum BlockNode {
    Breaking,
    Heading(HeadingNode),
    CodeBlock(CodeBlockNode),
    MathBlock(MathBlockNode),
    Paragraph(ParagraphNode),
}
