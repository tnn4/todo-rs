//! src/structs/mod.rs

/// enable other files within the module to access base file,
/// we only want the base struct to be used within
mod base; // -> src/to_do/structs/base.rs (private)

/// allow main file to access these 
pub mod done; // --> src/to_do/structs/done.rs
pub mod pending; // --> src/to_do/structs/pending.rs
pub mod traits; // --> traits/mod.rs