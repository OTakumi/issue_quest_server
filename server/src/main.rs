use rocket::launch;
use rocket::routes;

pub mod domain;
pub mod use_case {
    pub mod user;
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![use_case::user::get_user])
}
