use crate::domain::vo::entry::Tag as DomainTag;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Tag(String);

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

impl Into<DomainTag> for Tag {
    fn into(self) -> DomainTag {
        DomainTag::new(self.0.as_str())
    }
}
