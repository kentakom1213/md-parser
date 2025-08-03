//! リスト

use super::InlineNode;

#[derive(Debug)]
pub enum ListType {
    /// 番号なしのリスト
    List,
    /// 番号ありのリスト
    Enum,
}

#[derive(Debug)]
pub struct ListNode {
    pub list_type: ListType,
    pub contents: Vec<ListItem>,
}

#[derive(Debug)]
pub enum ListItem {
    Inline(InlineNode),
    List(ListNode),
}
