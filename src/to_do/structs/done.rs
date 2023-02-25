//! src/to_do/structs/done.rs

// Define done to-dos

// The super keyword refers to the parent scope
// We access the base struct through to_do/structs/mod.rs using super
use super::base::Base;

// we should allow the program to get,edit,delete by implementing 
// the traits: get,edit,delete in the `Done` struct
use super::traits::get::Get;


pub struct Done {
    pub super_struct: Base
}

impl Done {
    
    pub fn new(input_title: &str) -> Done {
        let base: Base = Base::new(input_title, "done");
        
        Done {
            super_struct: base,
        }
    }
}