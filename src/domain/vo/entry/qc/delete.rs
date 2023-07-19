use crate::domain::vo::entry::Id;

pub(crate) struct Delete {
    id: Id,
}

impl Delete {
    pub fn new(id: Id) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }
}
