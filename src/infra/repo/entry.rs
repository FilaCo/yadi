use crate::domain::entity::Entry;
use crate::domain::repo::Entry as EntryRepo;
use crate::domain::vo::entry::qc::{Create, Delete, Read, Update};
use crate::domain::vo::entry::Id;
use crate::domain::vo::Error as DomainError;
use crate::domain::Result as DomainResult;
use std::collections::HashMap;

pub struct HashMapEntry {
    entries: HashMap<Id, Entry>,
}

impl HashMapEntry {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}

impl EntryRepo for HashMapEntry {
    fn create(&mut self, qc: Create) -> DomainResult<Id> {
        if self.entries.contains_key(qc.id()) {
            return Err(DomainError::EntryAlreadyExists(qc.id().clone()));
        }

        self.entries.insert(
            qc.id().clone(),
            Entry::new(qc.id().clone(), qc.lifetime().clone()),
        );

        Ok(qc.id().clone())
    }

    fn read(&self, qc: &Read) -> DomainResult<&Entry> {
        let res = self.entries.get(qc.id());

        if let Some(entry) = res {
            Ok(entry)
        } else {
            Err(DomainError::EntryNotFound(qc.id().clone()))
        }
    }

    fn update(&mut self, qc: Update) -> DomainResult<Id> {
        if !self.entries.contains_key(qc.id()) {
            return Err(DomainError::EntryNotFound(qc.id().clone()));
        }

        self.entries.entry(qc.id().clone()).and_modify(|entry| {
            if qc.lifetime().is_some() {
                entry.set_lifetime(qc.lifetime().unwrap());
            }
        });

        Ok(qc.id().clone())
    }

    fn delete(&mut self, qc: &Delete) -> DomainResult<Entry> {
        if !self.entries.contains_key(qc.id()) {
            return Err(DomainError::EntryNotFound(qc.id().clone()));
        }

        Ok(self.entries.remove(qc.id()).unwrap())
    }
}

impl Default for HashMapEntry {
    fn default() -> Self {
        Self::new()
    }
}
