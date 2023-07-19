use crate::domain::vo::entry::{Id, Lifetime};

pub(crate) struct Update {
    id: Id,
    lifetime: Option<Lifetime>,
}

impl Update {
    pub fn new(id: Id, lifetime: Option<Lifetime>) -> Self {
        Self { id, lifetime }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn lifetime(&self) -> &Option<Lifetime> {
        &self.lifetime
    }
}
