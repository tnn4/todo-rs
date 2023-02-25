//! src/main.rs of web_app_1 
//! add documentation to the item that contains the comment rather than the item following the comment

use std::env;

/// declare module to_do
mod to_do;

/// to_do/structs/done.rs::Done
use to_do::structs::done::Done; // -> to_do/structs/done.rs::Done
/// to_do/structs/pending.rs::Pending
use to_do::structs::pending::Pending;

use crate::to_do::to_do_factory;
use crate::to_do::ItemTypes;

use to_do::structs::traits::create::Create; // you have tomimpor the Create trait to main or the compiler won't find it

/// read from file
use web_app_1::state::{read_json_file, write_to_file};
use serde_json::value::Value;
use serde_json::{Map, json};

fn main() {
    let to_do_item: Result<ItemTypes, &'static str> =
        to_do_factory(
            "pending", "washing");
    match to_do_item.unwrap() {
        ItemTypes::Pending(item) => item.create(
            &item.super_struct.title),
        
        ItemTypes::Done(item) => println!(
            "its a done item with the title: {}",
            item.super_struct.title)
    }


}

fn test_state() {
    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let title: &String = &args[2];

    // Read the data of file into string
    let mut state: Map<String, Value> = 
        read_json_file(&String::from("./state.json"));
    

    // Insert our data into the string
    state.insert(title.to_string(), json!(status));
    // Write our transformed string back to file
    if let () = write_to_file("./state.json", &mut state) {
        println!("The following was successfully written:\n{:?}", state);
    }
    else {
        println!("[ERROR] Could not write to file.");
    }
    
}

fn use_todo_module() {

    let done: Done = Done::new("shopping");

    let pending: Pending = Pending::new("laundry");
}

fn use_todo_factory_interface() {
    let to_do_item: Result<ItemTypes, &'static str> =
        to_do_factory(
            "pending", "make");

    match to_do_item.unwrap() {
        ItemTypes::Pending(item) => {
            println!("it's a pending item with the title: {}", item.super_struct.title)
        },
        ItemTypes::Done(item) => {
            println!("it")
        }
    }
}

#[test]
fn test() {
        /// we need to be able to pass parameters into our program
        let args: Vec<String> = env::args().collect();
        println!("{:?}", &args);
    
        let path: &str = &args[0];
        println!("path: {}", path);
    
        /// Check development mode: debug or release
        if path.contains("/debug/") {
            println!("Development app is running.");
        }
        else if path.contains("/release/") {
            println!("Production app is running.");
        }
}