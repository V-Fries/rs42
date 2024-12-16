mod macros;
mod result;

/// Trait implementations for pre-existing structs
pub mod extensions;
/// Structure that wraps a value and calls a callback function when it exits scope
pub mod scope_guard;

pub use result::Result;
