#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::tokio;
use rocket::State;

mod model;
mod criptography;

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
    // Verifique as credenciais do usuário (este é apenas um placeholder)
    if let Some(stored_hash) = db.get_password_hash(&login_request.username).await {
        match crypto::verify_password(&stored_hash, &login_request.password) {
            Ok(true) => {
                // Obtenha o número de telefone do usuário no banco de dados
                let phone = db.get_phone(&login_request.username).await.unwrap_or_else(|_| "+1234567890".to_string());
                let code = "123456"; // Gere um código real em produção

                match two_factor::send_sms(&phone, code).await {
                    Ok(_) => Json("Código enviado".to_string()),
                    Err(e) => Json(format!("Falha ao enviar o código: {}", e)),
                }
            }
            _ => Json("Usuário ou senha inválidos".to_string()),
        }
    } else {
        Json("Usuário ou senha inválidos".to_string())
    }
}*/

#[post("/verify_code", data = "<verify_request>")]
async fn verify_code(verify_request: Json<VerifyRequest>) -> Json<String> {
    let stored_code = "123456"; // armazenar o codigo aleatorio

    if model::verify_code(stored_code, &verify_request.code).await {
        Json("Sucesso na verificação".to_string())
    } else {
        Json("Falha na verificação".to_string())
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/auth", routes![verify_code])
}