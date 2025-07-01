//! インライン要素を表すノード

mod emph;
mod inlinecode;
mod inlinemath;
mod link;
mod plaintext;

use emph::EmphNode;
use inlinecode::InlineCodeNode;
use inlinemath::InlineMathNode;
use link::LinkNode;
use plaintext::PlainTextNode;

pub enum InlineNode {
    Emph(EmphNode),
    InlineMath(InlineMathNode),
    InlineCode(InlineCodeNode),
    Link(LinkNode),
    PlainText(PlainTextNode),
}
