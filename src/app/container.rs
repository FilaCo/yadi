use crate::app::result::Result;
use crate::domain::service::Entry as EntryService;
use crate::domain::vo::entry::{Id, Lifetime};
use crate::infra::dto::Tag;
use crate::infra::repo::HashMapEntry as HashMapEntryRepo;
use std::any::TypeId;

pub struct Container {
    entry_service: EntryService,
}

impl Container {
    pub fn new() -> Self {
        let entry_repo = Box::new(HashMapEntryRepo::default());

        let entry_service = EntryService::new(entry_repo);

        Self { entry_service }
    }

    pub fn resolve<T: 'static>(&self, tag: Option<&Tag>) -> Result<&T> {
        todo!()
    }

    pub fn remove<T: 'static>(&self, tag: Option<&Tag>) -> Result<T> {
        todo!()
    }

    pub fn add_singleton<T: 'static>(&mut self, tag: Option<&Tag>) -> Result<&mut Self> {
        self.add::<T>(Lifetime::Singleton, tag)
    }

    pub fn add_scoped<T: 'static>(&mut self, tag: Option<&Tag>) -> Result<&mut Self> {
        self.add::<T>(Lifetime::Scoped, tag)
    }

    pub fn add_transient<T: 'static>(&mut self, tag: Option<&Tag>) -> Result<&mut Self> {
        self.add::<T>(Lifetime::Transient, tag)
    }

    fn add<T: 'static>(&mut self, lifetime: Lifetime, tag: Option<&Tag>) -> Result<&mut Self> {
        let type_id = TypeId::of::<T>();
        let entry_id = Id::new(type_id, tag.and_then(|t| Some(t.clone().into())));

        self.entry_service
            .create(entry_id, lifetime)
            .map_err(|err| err.into())
            .and_then(|_| Ok(self))
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
