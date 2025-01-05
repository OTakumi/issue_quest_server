use anyhow;

use crate::domain::user::User;

pub trait UserRepository {
    async fn create(&self, user: &User) -> anyhow::Result<()>;
}
