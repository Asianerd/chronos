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

use account_handler::{AccountResult, Database};
use login_info::LoginInformation;

#[get("/")]
fn index() -> String {
    "can you understand me?".to_string()
}

#[get("/")]
fn debug(db: &State<Mutex<Database>>) -> String {
    let db = db.lock().unwrap();
    format!("{:?}", db)
}

#[get("/")]
fn load(db: &State<Mutex<Database>>) -> String {
    let mut db = db.lock().unwrap();
    *db = Database::load();
    "success".to_string()
}

#[get("/")]
fn save(db: &State<Mutex<Database>>) -> String {
    let mut db = db.lock().unwrap();
    db.save();
    "success".to_string()
}

// #region change get to post
#[post("/", data="<login>")]
fn login(db: &State<Mutex<Database>>, login: LoginInformation) -> String {
    let db = db.lock().unwrap();
    let result = db.login(&login);
    match result {
        // AccountResult::Success(_) => result.to_string(),
        // _ => format!("{{{result}:0}}")
        // AccountResult::Success(i) => utils::parse_response("success".to_string(), )

        AccountResult::Success(i) => format!("{{\"type\":\"success\",\"user_id\":{i}}}"),
        _ => format!("{{\"type\":{result},\"user_id\":0}}")
    }
}

#[post("/", data="<login>")]
fn signup(db: &State<Mutex<Database>>, login: LoginInformation) -> String {
    let mut db = db.lock().unwrap();
    let result = db.signup(&login);
    match result {
        AccountResult::Success(i) => format!("{{\"type\":\"success\",\"user_id\":{i}}}"),
        _ => format!("{{\"type\":{result},\"user_id\":0}}")
    }
}

#[post("/<start>/<end>", data="<login>")]
fn fetch_library(db: &State<Mutex<Database>>, login: LoginInformation, start: u128, end: u128) -> String {
    let db = db.lock().unwrap();
    let result = db.login(&login);
    match result {
        AccountResult::Success(user_id) => utils::parse_response(Some(serde_json::to_string(&db.users.get(&user_id).unwrap().fetch_library(start, end)).unwrap())),
        _ => utils::parse_response(None)
    }
}

#[post("/<r_species>/<r_time_species>/<start>/<end>/<r_occurance_species>/<repeating_day>/<title>/<description>/<colour>", data="<login>")]
fn add_task(
    db: &State<Mutex<Database>>,
    login: LoginInformation,
    r_species: String,
    r_time_species: String, start: u128, end: u128,
    r_occurance_species: String, repeating_day: u128,
    title: String,
    description: String,
    colour: u128
) -> String {
// fn add_task(db: &State<Mutex<Database>>, login: LoginInformation, r_species: String, r_time_species: String, repeating_day: u128, title: String, description: String, start: u128, end: Option<u128>, colour: u128) -> String {
    let mut db = db.lock().unwrap();
    let result = db.login(&login);
    match result {
        AccountResult::Success(user_id) => {
            let species = match task::Species::from_str(&r_species) {
                Ok(i) => i,
                Err(_) => return utils::parse_response(None)
            };
            let occurance_species = match task::OccuranceSpecies::from_str(&r_occurance_species) {
                Ok(i) => match i {
                    task::OccuranceSpecies::Repeating(_) => task::OccuranceSpecies::Repeating(repeating_day as u8),
                    _ => i
                },
                Err(_) => return utils::parse_response(None)
            };
            let time_species = match task::TimeSpecies::from_str(&r_time_species) {
                Ok(i) => match i {
                    task::TimeSpecies::Start(_) => task::TimeSpecies::Start(start),
                    task::TimeSpecies::Range(_, _) => task::TimeSpecies::Range(start, end),
                    task::TimeSpecies::AllDay(_) => task::TimeSpecies::AllDay(start),
                    task::TimeSpecies::DayRange(_, _) => task::TimeSpecies::DayRange(start, end)
                },
                Err(_) => return utils::parse_response(None)
            };
            db.users.get_mut(&user_id).unwrap().add_task(species, occurance_species, time_species, title, description, colour);
            db.save();
            utils::parse_response(Some("success".to_string()))
        },
        _ => utils::parse_response(None)
    }
}

#[post("/<task_id>/<week_start>/<current_day>", data="<login>")]
fn complete_task(db: &State<Mutex<Database>>, login: LoginInformation, task_id: usize, week_start: u128, current_day: u8) -> String {
    let mut db = db.lock().unwrap();
    let result = db.login(&login);
    match result {
        AccountResult::Success(user_id) => {
            db.save();
            utils::parse_response(Some(db.users.get_mut(&user_id).unwrap().complete_task(
                task_id as usize,
                week_start,
                current_day
            ).to_string()))
        },
        _ => utils::parse_response(None)
    }
}

#[post("/<task_id>", data="<login>")]
fn delete_task(db: &State<Mutex<Database>>, login: LoginInformation, task_id: usize) -> String {
    let mut db = db.lock().unwrap();
    let result = db.login(&login);
    match result {
        AccountResult::Success(user_id) => {
            let r = db.users.get_mut(&user_id).unwrap().delete_task(task_id as usize);
            utils::parse_response(Some(r.to_string()))
        },
        _ => utils::parse_response(None)
    }
}

#[post("/<task_id>/<r_species>/<r_time_species>/<start>/<end>/<r_occurance_species>/<repeating_day>/<title>/<description>/<colour>", data="<login>")]
fn update_task(
    db: &State<Mutex<Database>>,
    login: LoginInformation,
    task_id: usize,
    r_species: String,
    r_time_species: String, start: u128, end: u128,
    r_occurance_species: String, repeating_day: u128,
    title: String,
    description: String,
    colour: u128
) -> String {
    let mut db = db.lock().unwrap();
    let result = db.login(&login);
    match result {
        AccountResult::Success(user_id) => {
            let species = match task::Species::from_str(&r_species) {
                Ok(i) => i,
                Err(_) => return utils::parse_response(None)
            };
            let occurance_species = match task::OccuranceSpecies::from_str(&r_occurance_species) {
                Ok(i) => match i {
                    task::OccuranceSpecies::Repeating(_) => task::OccuranceSpecies::Repeating(repeating_day as u8),
                    _ => i
                },
                Err(_) => return utils::parse_response(None)
            };
            let time_species = match task::TimeSpecies::from_str(&r_time_species) {
                Ok(i) => match i {
                    task::TimeSpecies::Start(_) => task::TimeSpecies::Start(start),
                    task::TimeSpecies::Range(_, _) => task::TimeSpecies::Range(start, end),
                    task::TimeSpecies::AllDay(_) => task::TimeSpecies::AllDay(start),
                    task::TimeSpecies::DayRange(_, _) => task::TimeSpecies::DayRange(start, end)
                },
                Err(_) => return utils::parse_response(None)
            };
            db.users.get_mut(&user_id).unwrap().update_task(task_id, species, occurance_species, time_species, title, description, colour);
            utils::parse_response(Some("success".to_string()))
        },
        _ => utils::parse_response(None)
    }
}
// #endregion

#[launch]
fn rocket() -> _ {
    // rocket::build()
    rocket::custom(rocket::config::Config::figment().merge(("port", 8000)))
        // .manage(Mutex::new(data_instance::DataInstance::new()))
        .manage(Mutex::new(Database::load()))
        .mount("/save", routes![save])
        .mount("/load", routes![load])
        .mount("/debug", routes![debug])

        .mount("/login", routes![login])
        .mount("/sign_up", routes![signup])

        .mount("/add_task", routes![add_task])
        .mount("/complete_task", routes![complete_task])
        .mount("/fetch_library", routes![fetch_library])
        .mount("/delete_task", routes![delete_task])
        .mount("/update_task", routes![update_task])

        .mount("/", routes![index])

        .attach(cors::CORS)
}
