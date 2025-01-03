use rocket::launch;
use rocket::routes;
use rocket::*;

mod domain;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
