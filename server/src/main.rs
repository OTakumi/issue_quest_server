use rocket::launch;
use rocket::routes;

mod domain {
    pub mod user;
}

mod repository {
    pub mod user_repository;
}

mod controller {
    pub mod healthcheck;
    pub mod user;
}

mod usecase {
    pub mod create_user;
    pub mod get_user;
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![controller::healthcheck::healthcheck])
        .mount("/", routes![controller::user::get_user])
}
