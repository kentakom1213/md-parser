//! インライン要素

mod emph;
mod inlinecode;
mod inlinemath;
mod link;
mod plaintext;

pub use emph::EmphNode;
pub use inlinecode::InlineCodeNode;
pub use inlinemath::InlineMathNode;
pub use link::LinkNode;
pub use plaintext::PlainTextNode;

#[derive(Debug)]
pub enum InlineNode {
    Emph(EmphNode),
    InlineMath(InlineMathNode),
    InlineCode(InlineCodeNode),
    Link(LinkNode),
    PlainText(PlainTextNode),
}
