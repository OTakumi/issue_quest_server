use rocket::launch;
use rocket::routes;

use controller::{healthcheck, user};

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![healthcheck::healthcheck])
        .mount("/", routes![user::get_by_id])
}
