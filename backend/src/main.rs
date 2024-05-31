use std::{
    sync::Mutex,
    str::FromStr
};

// use data_instance::DataInstance;
// use std::time::Instant;
#[macro_use] extern crate rocket;
use rocket::State;

mod utils;
mod cors;

mod login_info;
mod task;
mod user;
mod account_handler;

use account_handler::Database;

#[get("/")]
fn index() -> String {
    "can you understand me?".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::custom(rocket::config::Config::figment().merge(("port", 8001)))
        .manage(Mutex::new(Database::load()))
        .mount("/save", routes![account_handler::save])
        .mount("/load", routes![account_handler::load])
        .mount("/debug", routes![account_handler::debug])

        .mount("/add_task", routes![task::add_task])
        .mount("/complete_task", routes![task::complete_task])
        .mount("/fetch_library", routes![task::fetch_library])
        .mount("/delete_task", routes![task::delete_task])
        .mount("/update_task", routes![task::update_task])

        .mount("/", routes![index])

        .attach(cors::CORS)
}
