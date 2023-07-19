use crate::domain::vo::entry::Id;

#[derive(Debug)]
pub(crate) enum Error {
    EntryNotFound(Id),
    EntryAlreadyExists(Id),
}
