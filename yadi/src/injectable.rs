use crate::Container;

/// Trait that marks a struct or enum as a possible member of a [DI Container](Container).
pub trait Injectable: 'static + Send + Sync {
    /// Method allows to initialize trait object from a [DI Container](Container).
    fn from_container(container: &Container) -> Self;
}
