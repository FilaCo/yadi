use crate::domain::vo::entry::Id;

pub(crate) struct Read {
    id: Id,
}

impl Read {
    pub fn new(id: Id) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }
}
