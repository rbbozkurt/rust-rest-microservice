use crate::dummy_data::User;
use rand::Rng;

/// Returns a vector of dummy users.
fn get_users() -> Vec<User> {
    vec![
        User {
            id: 1,
            name: "John Doe".to_string(),
            email: "johndoe@github.com".to_string(),
        },
        User {
            id: 2,
            name: "Jane Doe".to_string(),
            email: "janedoe@github.com".to_string(),
        },
        User {
            id: 3,
            name: "John Smith".to_string(),
            email: "johnsmit@github.com".to_string(),
        },
    ]
}
#[allow(dead_code)]
pub fn get_user_by_id(id: i32) -> Option<User> {
    let users = get_users();
    users.into_iter().find(|user| user.id == id)
}

pub fn get_random_user() -> User {
    let users = get_users();
    let random_index = rand::thread_rng().gen_range(0..users.len());
    users[random_index].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_by_id() {
        let user = get_user_by_id(2);
        assert_eq!(user.unwrap().name, "Jane Doe");
    }

    #[test]
    fn test_get_random_user() {
        let user = get_random_user();
        assert!(user.id >= 1 && user.id <= 3);
    }

    #[test]
    fn test_get_users() {
        let users = get_users();
        assert_eq!(users.len(), 3);
        assert_eq!(users[0].name, "John Doe");
        assert_eq!(users[1].name, "Jane Doe");
        assert_eq!(users[2].name, "John Smith");
    }
}
