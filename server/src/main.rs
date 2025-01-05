use rocket::launch;
use rocket::routes;

use controller::{healthcheck, user};

mod domain {
    pub mod user;
}

mod usecase {
    pub mod create_user;
    pub mod get_user;
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![healthcheck::healthcheck])
        .mount("/", routes![user::get_by_id])
}
