use crate::domain::vo::Error as DomainError;

#[derive(Debug)]
pub enum Error {}

impl From<DomainError> for Error {
    fn from(value: DomainError) -> Self {
        todo!()
    }
}
