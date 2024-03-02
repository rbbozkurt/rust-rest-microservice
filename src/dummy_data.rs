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
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the user.
    /// * `name` - The name of the user.
    /// * `email` - The email of the user.
    ///
    /// # Example
    /// let user = User::new(1, "John Doe", "john.doe@example.com");
    ///
    #[allow(dead_code)]
    pub fn new(id: i32, name: &str, email: &str) -> User {
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_user_new() {
        let name = "John Doe".to_string();
        let email = "johndoe@github.com".to_string();
        let id = 1;
        let user = User::new(id, &name, &email);

        assert!(id == user.id);
        assert!(name == user.name);
        assert!(email == user.email);
    }

    #[test]
    fn test_user_clone() {
        let name = "John Doe".to_string();
        let email = "johndoe@github.com".to_string();
        let id = 1;
        let user = User::new(id, &name, &email);
        let cloned_user = user.clone();

        assert!(id == cloned_user.id);
        assert!(name == cloned_user.name);
        assert!(email == cloned_user.email);
    }
}
