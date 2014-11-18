#![license = "MIT"]
#![deny(missing_docs, warnings)]
#![feature(tuple_indexing)]

//! Reduce over tuples of value-consumers for implementing strongly-typed
//! middleware-like systems.

/// Reducable objects.
pub trait Reduce<I, O> {
    /// Apply this to an input value, running it through potentially many places
    /// and returning the output.
    fn reduce(self, I) -> O;
}

mod impls;

