use crate::domain::vo::entry::{Id, Lifetime};

pub(crate) struct Entry {
    id: Id,
    lifetime: Lifetime,
}

impl Entry {
    pub fn new(id: Id, lifetime: Lifetime) -> Self {
        Self { id, lifetime }
    }

    pub fn set_lifetime(&mut self, lifetime: Lifetime) -> &mut Self {
        self.lifetime = lifetime;

        self
    }
}
