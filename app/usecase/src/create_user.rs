use domain::user;

pub struct CreateUser {
    pub name: String,
    pub email: String,
}

pub async fn create_user(cmd: CreateUser) -> anyhow::Result<User> {
    let user = User::new(cmd.name, cmd.email);
    Ok(user)
}
