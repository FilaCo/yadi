#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub(crate) enum Lifetime {
    Scoped,
    Singleton,
    Transient,
}
