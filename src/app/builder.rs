use crate::{Config, Tag};

pub struct Builder {}

impl Builder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn add_scoped<T>(&mut self, tag: Option<&Tag>) -> &mut Self {
        self
    }

    pub fn add_singleton<T>(&mut self, tag: Option<&Tag>) -> &mut Self {
        self
    }

    pub fn add_transient<T>(&mut self, tag: Option<&Tag>) -> &mut Self {
        self
    }

    pub fn build(self) -> crate::Result<Config> {
        todo!()
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}
