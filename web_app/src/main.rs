#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::tokio;
use rocket::State;

mod model;

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct VerifyRequest {
    phone: String,
    code: String,
}

/*#[post("/login", data = "<login_request>")]
async fn login(login_request: Json<LoginRequest>, db: &State<Database>) -> Json<String> {
    // atribuir do front para LoginRequest o username e o password e fazer a conferencia de baixo no bd
    if login_request.username == "User do bd" && login_request.password == "password do bd" {
        // pegar o telefone do bd
        let phone = "";

        let code = "123456"; // fazer algo pra gerar o codigo aleatorio(essa parte eu faço)

        match model::send_sms(&phone, code).await {
            Ok(_) => Json("2FA code sent".to_string()),
            Err(e) => Json(format!("Failed to send 2FA code: {}", e)),
        }
    } else {
        Json("Invalid username or password".to_string())
    }
}*/

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
        .mount("/auth", routes![verify_code])
}