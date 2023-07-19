use crate::domain::vo::entry::Tag;
use std::any::TypeId;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub(crate) struct Id {
    type_id: TypeId,
    tag: Option<Tag>,
}

impl Id {
    pub fn new(type_id: TypeId, tag: Option<Tag>) -> Self {
        Self { type_id, tag }
    }
}
