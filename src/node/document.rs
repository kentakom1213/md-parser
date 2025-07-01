//! ドキュメント全体を表すノード

use super::BlockNode;

#[derive(Debug)]
pub struct DocumentNode {
    /// ブロックの列
    pub blocks: Vec<BlockNode>,
}
