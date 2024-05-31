use std::{sync::Mutex, str::FromStr};

use rocket::State;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::{account_handler::{AccountResult, Database}, user::{LoginInformation, LoginResult}, utils};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Task {
    // unique per user
    // user a : 0...∞
    // user b : 0...∞
    pub id: usize,
    pub species: Species,
    pub occurance_species: OccuranceSpecies,
    pub time_species: TimeSpecies,
    pub title: String,
    pub description: String,
    pub colour: u128,
}
impl Task {
    pub fn in_range(&self, start: u128, end: u128) -> bool {
        let near: u128 = self.start_time();

        match self.end_time() {
            Some(far) => (start <= far) && (near <= end),
            None => (near >= start) && (end >= near)
        }
    }

    pub fn start_time(&self) -> u128 {
        // for qol
        match self.time_species {
            TimeSpecies::Start(s) => s,
            TimeSpecies::Range(s, _) => s,
            TimeSpecies::AllDay(s) => s,
            TimeSpecies::DayRange(s, _) => s
        }
    }

    pub fn end_time(&self) -> Option<u128> {
        // for qol
        match self.time_species {
            TimeSpecies::Start(_) => None,
            TimeSpecies::Range(_, e) => Some(e),
            TimeSpecies::AllDay(_) => None,
            TimeSpecies::DayRange(_, e) => Some(e)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, EnumString)]
pub enum Species {
    Task(bool),
    Event
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, EnumString)]
pub enum TimeSpecies {
    // IF OCCURANCE IS REPEATING, START AND END IS SECONDS ELAPSED SINCE START OF THE DAY
    // THIS IS HANDLED ONLY IN THE FRONTEND
    // NO WAY TO CORRECT FOR TIMEZONES IN THE BACKEND

    // from 12:30 18 Apr 2024 to 5:00 19 Apr 2024
    Start(u128),
    Range(u128, u128),

    // from 1 Jan 1970 to 2 Jan 1970
    // stored in epoch unix
    // 0 -> 1 Jan 1970
    // 86400 -> 2 Jan 1970
    AllDay(u128), // ignore u128 if occurance is repeating
    DayRange(u128, u128) // <- cant have day range if occurance is repeating
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, EnumString)]
pub enum OccuranceSpecies {
    Once, // occurs once only

    // start and end in time_species must be in the same day
    Repeating(u8) // days of the week to repeat
    // eg : 10010000 gym every monday and thursday
    // ignore first bit 
}

// #region api calls
#[post("/<start>/<end>", data="<login>")]
pub fn fetch_library(db: &State<Mutex<Database>>, login: LoginInformation, start: u128, end: u128) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(user_id) => utils::parse_response(Ok(serde_json::to_string(&db.users.get(&user_id).unwrap().fetch_library(start, end)).unwrap())),
        e => utils::parse_response(Err(e))
    }
}

#[post("/<r_species>/<r_time_species>/<start>/<end>/<r_occurance_species>/<repeating_day>/<title>/<description>/<colour>", data="<login>")]
pub fn add_task(
    db: &State<Mutex<Database>>,
    login: LoginInformation,
    r_species: String,
    r_time_species: String, start: u128, end: u128,
    r_occurance_species: String, repeating_day: u128,
    title: String,
    description: String,
    colour: u128
) -> String {
// pub fn add_task(db: &State<Mutex<Database>>, login: LoginInformation, r_species: String, r_time_species: String, repeating_day: u128, title: String, description: String, start: u128, end: Option<u128>, colour: u128) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(user_id) => {
            let species = match Species::from_str(&r_species) {
                Ok(i) => i,
                Err(_) => Species::Event
            };
            let occurance_species = match OccuranceSpecies::from_str(&r_occurance_species) {
                Ok(i) => match i {
                    OccuranceSpecies::Repeating(_) => OccuranceSpecies::Repeating(repeating_day as u8),
                    _ => i
                },
                Err(_) => OccuranceSpecies::Once
            };
            let time_species = match TimeSpecies::from_str(&r_time_species) {
                Ok(i) => match i {
                    TimeSpecies::Start(_) => TimeSpecies::Start(start),
                    TimeSpecies::Range(_, _) => TimeSpecies::Range(start, end),
                    TimeSpecies::AllDay(_) => TimeSpecies::AllDay(start),
                    TimeSpecies::DayRange(_, _) => TimeSpecies::DayRange(start, end)
                },
                Err(_) => TimeSpecies::Start(start)
            };
            db.users.get_mut(&user_id).unwrap().add_task(species, occurance_species, time_species, title, description, colour);
            db.save();
            utils::parse_response(Ok("success".to_string()))
        },
        e => utils::parse_response(Err(e))
    }
}

#[post("/<task_id>/<week_start>/<current_day>", data="<login>")]
pub fn complete_task(db: &State<Mutex<Database>>, login: LoginInformation, task_id: usize, week_start: u128, current_day: u8) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(user_id) => {
            db.save();
            utils::parse_response(Ok(db.users.get_mut(&user_id).unwrap().complete_task(
                task_id as usize,
                week_start,
                current_day
            ).to_string()))
        },
        e => utils::parse_response(Err(e))
    }
}

#[post("/<task_id>", data="<login>")]
pub fn delete_task(db: &State<Mutex<Database>>, login: LoginInformation, task_id: usize) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(user_id) => {
            let r = db.users.get_mut(&user_id).unwrap().delete_task(task_id as usize);
            utils::parse_response(Ok(r.to_string()))
        },
        e => utils::parse_response(Err(e))
    }
}

#[post("/<task_id>/<r_species>/<r_time_species>/<start>/<end>/<r_occurance_species>/<repeating_day>/<title>/<description>/<colour>", data="<login>")]
pub fn update_task(
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
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(user_id) => {
            let species = match Species::from_str(&r_species) {
                Ok(i) => i,
                Err(_) => Species::Event
            };
            let occurance_species = match OccuranceSpecies::from_str(&r_occurance_species) {
                Ok(i) => match i {
                    OccuranceSpecies::Repeating(_) => OccuranceSpecies::Repeating(repeating_day as u8),
                    _ => i
                },
                Err(_) => OccuranceSpecies::Once
            };
            let time_species = match TimeSpecies::from_str(&r_time_species) {
                Ok(i) => match i {
                    TimeSpecies::Start(_) => TimeSpecies::Start(start),
                    TimeSpecies::Range(_, _) => TimeSpecies::Range(start, end),
                    TimeSpecies::AllDay(_) => TimeSpecies::AllDay(start),
                    TimeSpecies::DayRange(_, _) => TimeSpecies::DayRange(start, end)
                },
                Err(_) => TimeSpecies::Start(start)
            };
            db.users.get_mut(&user_id).unwrap().update_task(task_id, species, occurance_species, time_species, title, description, colour);
            utils::parse_response(Ok("success".to_string()))
        },
        e => utils::parse_response(Err(e))
    }
}
// #endregion
