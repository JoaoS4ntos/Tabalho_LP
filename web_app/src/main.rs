#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate rocket_sync_db_pools;


use rocket::fs::{FileServer, relative};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::tokio;
use rocket::State;
use rocket::form::Form;
use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel::PgConnection;
use rocket_sync_db_pools::Connection;
use std::fs;
use diesel::prelude::*;
use rocket::response::content::RawHtml;

mod model;
mod criptography;
mod pages;
mod schema;

use crate::model::NewUser;

#[database("postgres_db")]
struct DbConn(PgConnection);

#[derive(FromForm)]
struct UserForm {
    username: String,
    password: String,
    phone: String,
}

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

#[get("/register")]
async fn get_register() -> Option<RawHtml<String>> {
    let content: String = fs::read_to_string("src/static/register.html").ok()?;
    Some(RawHtml(content))
}

#[post("/register", data = "<user_form>")]
async fn post_register(user_form: Form<UserForm>, conn: DbConn) -> Result<RawHtml<String>, rocket::http::Status> {
    use crate::schema::users::dsl::users;

    let new_user = NewUser {
        username: user_form.username.clone(),
        password: user_form.password.clone(),
        phone: user_form.phone.clone(),
    };

    let result = conn.run(move |c| {
        diesel::insert_into(users)
            .values(&new_user)
            .execute(c)
    }).await;

    match result {
        Ok(_) => Ok(RawHtml(format!("<p>User '{}' registered successfully!</p>", user_form.username))),
        Err(e) => Ok(RawHtml(format!("<p>Error registering user: {}</p>", e))),
    }
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
        .attach(DbConn::fairing())
        .mount("/", routes![get_register, post_register])
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/", FileServer::from(relative!("static")))
        .mount("/auth", routes![verify_code])
        .mount("/", routes![pages::register_page, pages::cloud_page])
}