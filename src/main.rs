use rocket::{get, http::Status, serde::json::Json};
use serde::Serialize;

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub status: String,
}

#[get("/health")]
pub async fn health_check_handler() -> Result<Json<HealthCheckResponse>, Status> {
    let response_json = HealthCheckResponse {
        status: "ok".to_string(),
    };
    Ok(Json(response_json))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health_check_handler,])
}
