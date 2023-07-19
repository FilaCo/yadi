use crate::infra::dto::Error;

pub type Result<T> = std::result::Result<T, Error>;
