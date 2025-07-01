use md_parser::node::{
    BlockNode, DocumentNode, HeadingNode, InlineNode, ParagraphNode,
    inline::{LinkNode, PlainTextNode},
};

fn main() {
    let doc: DocumentNode = DocumentNode {
        blocks: vec![
            BlockNode::Heading(HeadingNode {
                level: 1,
                contents: "こんにちは".to_string(),
            }),
            BlockNode::ParagraphNode(ParagraphNode {
                contents: vec![
                    InlineNode::PlainText(PlainTextNode {
                        contents: "こんにちは．これがMarkDownの構文木です．".to_string(),
                    }),
                    InlineNode::Link(LinkNode {
                        contents: "GitHubリポジトリ".to_string(),
                        href: "https://github.com/kentakom1213/md-parser".to_string(),
                    }),
                ],
            }),
        ],
    };

    println!("{doc:#?}");
}
