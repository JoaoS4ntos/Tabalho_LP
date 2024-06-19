#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "sssss"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/login", routes![index])

}