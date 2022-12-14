#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::request::Form;
use rocket::http::RawStr;
use rocket::response::content::Html;

#[derive(FromForm)]
struct UserInput<'f> {
    // The raw, undecoded value. You _probably_ want `String` instead.
    value: &'f RawStr
}

#[post("/submit", data = "<user_input>")]
fn submit_task(user_input: Form<UserInput>) -> String {
    format!("Your value: {}", user_input.value)
}

#[get("/")]
fn hello() -> rocket::response::content::Html<&'static str> {
    Html(include_str!("main.html"))
}

fn main() {
    rocket::ignite()
    .mount("/", routes![hello])
    .mount("/", routes![submit_task])
    .launch();
}