use crate::domain::vo::{EntryId, LifeTime};

pub(crate) struct Entry {
    id: EntryId,
    life_time: LifeTime,
}

impl Entry {
    pub fn new(id: EntryId, life_time: LifeTime) -> Self {
        Self { id, life_time }
    }
}
