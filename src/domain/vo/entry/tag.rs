#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub(crate) struct Tag(String);

impl Tag {
    pub fn new(raw_tag: &str) -> Self {
        Self(raw_tag.to_string())
    }
}

impl From<&str> for Tag {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}
