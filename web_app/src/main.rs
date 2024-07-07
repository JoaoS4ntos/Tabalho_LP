#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::tokio;

mod model;

#[derive(Deserialize)]
struct VerifyRequest {
    phone: String,
    code: String,
}

#[get("/send_code/<phone>")]
async fn send_code(phone: String) -> Json<String> {
    let code = "123456"; // fazer um gerador de codigo aleatorio

    match model::send_sms(&phone, code).await {
        Ok(_) => Json("Code sent".to_string()),
        Err(e) => Json(format!("Failed to send SMS: {}", e)),
    }
}

#[post("/verify_code", data = "<verify_request>")]
async fn verify_code(verify_request: Json<VerifyRequest>) -> Json<String> {
    let stored_code = "123456"; // armazenar o codigo aleatorio

    if model::verify_code(stored_code, &verify_request.code).await {
        Json("Verification successful".to_string())
    } else {
        Json("Verification failed".to_string())
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/2fa", routes![send_code, verify_code])
}