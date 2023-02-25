//! src/to_do/mod.rs

// Now that we have the structs we need, we can define them here to allow the main file to use them

// Make structs publicly available by defining them here

/// src/to_do/structs/mod.rs
pub mod structs; // -> src/to_do/structs/mod.rs

use structs::done::Done;
use structs::pending::Pending;

/// mod.rs
/// ```
/// mod base; // -> base.rs
/// pub mod done; -> done.rs
/// pub mod pending; -> pending.rs
/// ```
fn mod_path_example(){}

pub enum ItemTypes {
    Pending (Pending),
    Done(Done)
}

/// Create a pending or done todo item 
pub fn to_do_factory(
    item_type: &str,
    item_title: &str) -> Result<ItemTypes, &'static str> {
        // create item based on which one was picked
        if item_type == "pending" {
            let pending_item = Pending::new(item_title);
            Ok(ItemTypes::Pending(pending_item))
        }
        else if item_type == "done" {
            let done_item = Done::new(item_title);
            Ok(ItemTypes::Done(done_item))
        }
        else {
            Err("This is not accepted.")
        }
        
}

/// alias for to_do_factory(item_type: &str, item_title: &str)
pub fn create_todo_item(
    item_type: &str,
    item_title: &str) -> Result<ItemTypes, &'static str> {
        to_do_factory(item_type, item_title)
}