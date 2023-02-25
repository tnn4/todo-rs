## Managing structs with factories

We can build an interface with the factory pattern.

This is where we select the right struct based on input, build it, return it.

to_do/mod.rs
``` rs
pub mod structs;

use structs::done::Done;
use structs::pending::Pending;

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
```

## Defining functionality with traits
Inside the `structs` directory make a `traits` directory
and create `mod.rs` `create.rs` `delete.rs` `edit.rs` `get.rs`

These files will hold the functionality for our to do.

tree src
```
src
├── lib.rs
├── main.rs
└── to_do
    ├── mod.rs
    └── structs
        ├── base.rs
        ├── done.rs
        ├── mod.rs
        ├── pending.rs
        └── traits <-- create directory here
            ├── create.rs
            ├── delete.rs
            ├── edit.rs
            ├── get.rs
            └── mod.rs
```

Now that the structs are enhanced with traits we can now analyze their scalability.

If we add another to-do item struct, we can slot in a range of traits to instantly give it functionality.

# Interacting with the environment

## Reading and writing JSON files
Install serde: `cargo add serde`
or add to `cargo.toml`
```
serde_json = {
    version = "1.0",
    default-features = false,
    features = ["alloc"]
}
```

We need to be able to read and write from files so add a state.rs file with read and write methods.

