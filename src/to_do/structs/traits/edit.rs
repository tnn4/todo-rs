//! src/to_do/structs/traits/edit.rs
//! 
//! 

use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

// use todo::state::write_to_file // doesn't work
use crate::state::write_to_file;

/// For editing we need to be able set a item to pending
/// and done states
pub trait Edit {
    // allow setting todo item to done
    fn set_to_done(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(String::from("done")));
        write_to_file("./state.json", state);
        println!("\n\n{} is being set to done\n\n", title);
    }
    // allow setting todo item to pending
    fn set_to_pending(&self, title:&str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!("pending".to_string()));
        write_to_file("./state.json", state);
        println!("{} is being set to done", title);
    }
}