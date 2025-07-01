//! 地の文を表すノード

use crate::parser::Parse;

#[derive(Debug, PartialEq)]
pub struct PlainTextNode {
    pub contents: String,
}

impl Parse for PlainTextNode {
    fn parse(s: &str) -> Option<(Self, &str)> {
        Some((
            PlainTextNode {
                contents: s.to_string(),
            },
            "",
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::parser::Parse;

    use super::PlainTextNode;

    #[test]
    fn test_parse_plaintext() {
        let s = "hello";

        assert_eq!(
            PlainTextNode::parse(s),
            Some((
                PlainTextNode {
                    contents: "hello".to_string()
                },
                ""
            ))
        )
    }
}
