use crate::domain::entity::Entry as EntryEntity;
use crate::domain::vo::entry::qc::{Create, Delete, Read, Update};
use crate::domain::vo::entry::Id;
use crate::domain::Result;

pub(crate) trait Entry {
    fn create(&mut self, qc: Create) -> Result<Id>;
    fn read(&self, qc: &Read) -> Result<&EntryEntity>;
    fn update(&mut self, qc: Update) -> Result<Id>;
    fn delete(&mut self, qc: &Delete) -> Result<EntryEntity>;
}
