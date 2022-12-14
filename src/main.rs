#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use controller::{turn_on, turn_off};
use rocket::request::Form;
use rocket::http::RawStr;
use rocket::response::content::Html;
pub mod controller;

#[derive(FromForm)]
struct UserInput {
    // The raw, undecoded value. You _probably_ want `String` instead.
    onoff: String
}

#[post("/submit", data = "<user_input>")]
fn submit_task(user_input: Form<UserInput>) -> String {
    if user_input.onoff.to_string() == "on" {
        turn_on();
    }

    else if user_input.onoff.to_string() == "off" {
        turn_off()
    }

    return user_input.onoff.to_string()
}

#[get("/")]
fn index() -> rocket::response::content::Html<&'static str> {
    Html(include_str!("main.html"))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![submit_task])
            .launch();
}