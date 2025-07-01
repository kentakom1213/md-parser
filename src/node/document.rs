//! ドキュメント全体を表すノード

use super::BlockNode;

pub struct DocumentNode {
    /// ブロックの列
    pub blocks: Vec<BlockNode>,
}
