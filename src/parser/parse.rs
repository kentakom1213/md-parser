//! 解析を行う

pub trait Parse
where
    Self: Sized,
{
    fn parse(s: &str) -> Option<(Self, &str)>;
}
