//! src/to_do/structs/base.rs

/// base struct
pub struct Base {
    pub title: String,
    pub status: String,
}

/// functions for base structs
impl Base {
    pub fn new(input_title: &str, input_status: &str) -> Base {
        return Base {
                title: input_title.to_string(),
                status: input_status.to_string(),
        }
    }
}