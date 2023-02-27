//! src/to_do/structs/traits/edit.rs
//! 

use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

// use todo::state::write_to_file // doesn't work
use crate::state::write_to_file;

pub trait Create {
    fn create(&self, title: &str, status: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(status));
        write_to_file("./state_json", state);
        println!("\n\n{} is being created\n\n", title);
    }
}