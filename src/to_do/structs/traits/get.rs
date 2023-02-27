//! src/to_do/structs/traits/get.rs
//! 

use serde_json::Map;
use serde_json::value::Value;

/// This function prints the process
pub trait Get {
    // here we bind the `get` function with &self parameters so it becomes a method
    // this enables the struct to call the function directly
    fn get (&self, title: &str, state: &Map<String, Value>) {
        let item: Option<&Value> = state.get(title);
        match item {
            Some(result) => {
                println!("\n\nItem: {}", title);
                println!("Status: {}\n\n", result);
            },
            None => println!("item: {} was not found", title)
        }
        // println!("{} is being fetched", title);
    }
}