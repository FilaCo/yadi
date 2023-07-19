use crate::domain::vo::entry::{Id, Lifetime};

pub(crate) struct Create {
    id: Id,
    lifetime: Lifetime,
}

impl Create {
    pub fn new(id: Id, lifetime: Lifetime) -> Self {
        Self { id, lifetime }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn lifetime(&self) -> &Lifetime {
        &self.lifetime
    }
}
