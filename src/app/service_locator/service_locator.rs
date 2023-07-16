use crate::app::service_locator::Builder;
use crate::infra::dto::Tag;
use std::sync::Arc;

pub struct ServiceLocator {}

impl ServiceLocator {
    pub fn new(builder: Builder) -> Self {
        Self {}
    }

    pub fn builder() -> Builder {
        Builder::default()
    }

    pub async fn get<'a, T>(&self, tag: Option<&Tag<'a>>) -> Option<Arc<T>> {
        todo!()
    }
}
