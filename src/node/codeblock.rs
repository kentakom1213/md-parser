//! コードブロックを表すノード

pub enum CodeLanguage {
    Python,
    Rust,
}

pub struct CodeBlockNode {
    pub language: CodeLanguage,
    pub contents: String,
}
