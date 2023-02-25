pub mod state;

use std::env;

/// Check development mode:debug or release
pub fn check_development_mode() {
           
    let path = get_path();

    if path.contains("/debug/") {
        println!("Development app is running.");
    }
    else if path.contains("/release/") {
        println!("Production app is running.");
    }
}

/// get arguments from command line
pub fn get_cli_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}

/// get the path of the binary
/// env::args() -> std::env::Args - an iterator over the arguments of a process, yields `String` value for each argument
/// std::iter::Iterator::collect() -> transforms an iterator into a collection
/// use turbofish ::<> for type annotation
pub fn get_path() -> String{

    // implicitly moving out of a `Vec` is not allowed as it would leave it an invalid state
    // one element is moved out, the others are not
    // you need to clone the element at the index you want
    env::args().collect::<Vec<String>>()[0].clone() // this is an example of using the turbofish: ::<>

}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
