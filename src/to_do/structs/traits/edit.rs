//! src/to_do/structs/traits/edit.rs
//! 
//! 


pub trait Edit {
    // allow setting todo item to done
    fn set_to_done(&self, title: &str) {
        println!("{} is being set to done", title);
    }
    // allow setting todo item to pending
    fn set_to_pending(&self, title:&str) {
        println!("{} is being set to done", title);
    }
}