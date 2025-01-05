use anyhow::bail;

use ulid::Ulid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        User {
            id: UserId::new(),
            name,
            email,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UserId(Ulid);

impl UserId {
    pub fn new() -> Self {
        UserId(Ulid::new())
    }
}

#[derive(Debug, Clone)]
pub struct UserName(String);

impl UserName {
    pub fn new(name: String) -> anyhow::Result<Self> {
        if name.is_empty() {
            bail!("name is empty");
        }

        Ok(UserName(name))
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

    #[test]
    fn test_empty_name() {
        let result = UserName::new("".to_string());
        assert!(result.is_err());
    }
}
