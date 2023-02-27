//! todo/src/processes.rs
//! The purpose of os this module is to direct the flow of commands
//! We need an entry point to process the input and direct it to the right function to process the item.

use serde_json::Map;
use serde_json::value::Value;

// super = parent of the current module
// So this is a submodule
use crate::to_do::ItemTypes; // todo/src/to_do/mod.rs
use todo::to_do::structs::done::Done;
