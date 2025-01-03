use ulid::Ulid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Ulid,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        User {
            id: Ulid::new(),
            name,
            email,
        }
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_new() {
        let user = User::new("Alice".to_string(), "test@sample.co.jp".to_string());
        println!("{:?}", user);
        assert_eq!(user.name, "Alice");
        assert_eq!(user.email, "test@sample.co.jp");
    }
}
