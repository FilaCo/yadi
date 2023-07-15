use crate::{Builder, Tag};

pub struct Config {}

impl Config {
    pub fn new(builder: Builder) -> Self {
        Self {}
    }

    pub fn builder() -> Builder {
        Builder::default()
    }

    pub async fn get<'a, T>(&self, tag: Option<&Tag<'a>>) -> Option<&T> {
        todo!()
    }
}
