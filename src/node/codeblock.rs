//! コードブロックを表すノード

#[derive(Debug)]
pub enum CodeLanguage {
    Python,
    Rust,
}

#[derive(Debug)]
pub struct CodeBlockNode {
    pub language: CodeLanguage,
    pub contents: String,
}
