use rocket::get;
use rocket::serde::json::Json;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetUser {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[get("/user")]
pub fn get_by_id() -> Json<GetUser> {
    Json(GetUser {
        id: 1,
        name: "John Doe".to_string(),
        email: "test@sample.co.jp".to_string(),
    })
}
