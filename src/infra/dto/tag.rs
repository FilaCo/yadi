pub struct Tag<'a>(&'a str);

impl<'a> Tag<'a> {
    pub fn new(raw_tag: &'a str) -> Self {
        Self(raw_tag)
    }
}

impl<'a> From<&'a str> for Tag<'a> {
    fn from(value: &'a str) -> Self {
        Self::new(value)
    }
}
