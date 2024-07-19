use std::sync::Arc;

use crate::prelude::{Builder, Injectable};

pub trait Container {
    fn builder() -> impl Builder;
    fn resolve<T: Injectable>(&self) -> Arc<T>;
}
