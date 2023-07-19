use crate::domain::entity::Entry as EntryEntity;
use crate::domain::repo::Entry as EntryRepo;
use crate::domain::vo::entry::qc::{Create, Delete, Read};
use crate::domain::vo::entry::{Id, Lifetime};
use crate::domain::Result;

pub(crate) struct Entry {
    entry_repo: Box<dyn EntryRepo>,
}

impl Entry {
    pub fn new(entry_repo: Box<dyn EntryRepo>) -> Self {
        Self { entry_repo }
    }

    pub fn create(&mut self, entry_id: Id, lifetime: Lifetime) -> Result<Id> {
        self.entry_repo.create(Create::new(entry_id, lifetime))
    }

    pub fn delete(&mut self, entry_id: Id) -> Result<EntryEntity> {
        self.entry_repo.delete(&Delete::new(entry_id))
    }
}
