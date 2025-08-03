//! ドキュメント全体

use super::BlockNode;

#[derive(Debug)]
pub struct DocumentNode {
    /// ブロックの列
    pub blocks: Vec<BlockNode>,
}
