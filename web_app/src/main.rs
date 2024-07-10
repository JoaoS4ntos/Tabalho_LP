#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate rocket_sync_db_pools;


use rocket::fs::{FileServer, relative};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::State;
use rocket::form::Form;
use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel::PgConnection;
use std::fs;
use diesel::prelude::*;
use rocket::response::content::RawHtml;
use dotenvy::dotenv;
use std::env;
use rocket::fs::NamedFile;
use crate::db::DbPool; // Importe o DbPool do seu módulo de banco de dados

mod model;
mod criptography;
mod pages;
mod schema;
mod db;
mod models;
mod user_ops;
mod files_ops;
mod args;

use crate::model::NewUser;

#[database("postgres_db")]
struct DbConn(PgConnection);

#[derive(FromForm)]
struct UserForm {
    username: String,
    password: String,
    phone: String,
}

#[derive(FromForm)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct VerifyRequest {
    phone: String,
    code: String,
}

#[get("/register", rank = 3)]
async fn get_register() -> Option<RawHtml<String>> {
    let content: String = fs::read_to_string("src/static/register.html").ok()?;
    Some(RawHtml(content))
} 

#[post("/register", data = "<user_form>")]
async fn post_register(user_form: Form<UserForm>, conn: DbConn) -> Result<RawHtml<String>, rocket::http::Status> {

    use crate::schema::users::dsl::users;

    let new_user: NewUser = NewUser {
        username: user_form.username.clone(),
        password_hash: user_form.password.clone(),
        phone: user_form.phone.clone(),
    };

    let result: Result<usize, diesel::result::Error> = conn.run(move |c: &mut PgConnection| {
        diesel::insert_into(users)
            .values(&new_user)
            .execute(c)
    }).await;

    match result {
        Ok(_) => Ok(RawHtml(format!("<p>User '{}' registered successfully!</p>", user_form.username))),
        Err(e) => Ok(RawHtml(format!("<p>Error registering user: {}</p>", e))),
    }
}

#[post("/login", data = "<login_request>")]
async fn login(login_request: Form<LoginRequest>, db: &State<DbPool>) -> Json<String> {
    let login_request = login_request.into_inner();
    
    // Obtendo a conexão do pool
    let mut conn = db
        .get()
        .expect("Failed to get DB connection from pool.");

    // Verificando as credenciais do usuário
    if let Some(user) = models::get_user_by_username(&mut conn, &login_request.username) {
        if bcrypt::verify(&login_request.password, &user.password_hash).unwrap_or(false) {
            // Obtendo o número de telefone do usuário no banco de dados
            let phone = user.phone.clone();
            let code = "123456"; // Gere um código real em produção

            // Simulando o envio do código por SMS
            match model::send_sms(&phone, code).await {
                Ok(_) => Json("Código enviado".to_string()),
                Err(e) => Json(format!("Falha ao enviar o código: {}", e)),
            }
        } else {
            Json("Usuário ou senha inválidos".to_string())
        }
    } else {
        Json("Usuário ou senha inválidos".to_string())
    }
}

#[post("/verify_code", data = "<verify_request>")]
async fn verify_code(verify_request: Json<VerifyRequest>) -> Json<String> {
    let stored_code = "123456"; // armazenar o codigo aleatorio

    if model::verify_code(stored_code, &verify_request.code).await {
        Json("Sucesso na verificação".to_string())
    } else {
        Json("Falha na verificação".to_string())
    }
}

#[get("/", rank = 4)]
async fn index_page() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![get_register, post_register,index_page, pages::register_page, pages::cloud_page])
        .mount("/static", FileServer::from(relative!("static")).rank(10))
        .mount("/auth", routes![login, verify_code])
}