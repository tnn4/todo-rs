//! src/to_do/structs/traits/get.rs
//! 


/// This function prints the process
pub trait Get {
    // here we bind the `get` function with &self parameters so it becomes a method
    // this enables the struct to call the function directly
    fn get (&self, title: &str) {
        println!("{} is being fetched", title);
    }
}