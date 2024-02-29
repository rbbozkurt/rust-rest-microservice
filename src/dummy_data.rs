use std::clone::Clone;

use serde::{Deserialize, Serialize};

/// Represents a user with an ID, name, and email.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl User {
    /// Returns a new `User` instance with the given ID, name, and email.
    #[allow(dead_code)]
    pub fn new(id: i32, name: &str, email: &str) -> User {
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
        }
    }
}