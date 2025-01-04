use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HealthCheck {
    status: &'static str,
}

#[get("/healthcheck")]
pub fn healthcheck() -> Json<HealthCheck> {
    Json(HealthCheck { status: "ok" })
}
