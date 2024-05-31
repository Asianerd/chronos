use std::{collections::HashMap, fmt};

use rocket::data;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::{data::FromData, Data};
use rocket::request::Request;
use serde::{Deserialize, Serialize};

use crate::{account_handler::Database, soterius, task};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct User {
    pub id: u128,
    pub username: String,
    pub library: HashMap<usize, task::Task>,
    // pub repeating_tasks: Vec<task::Task>
}
impl User {
    pub fn add_task(&mut self, species: task::Species, occurance_species: task::OccuranceSpecies, time_species: task::TimeSpecies, title: String, description: String, colour: u128) {
        let id = self.generate_task_id();
        self.library.insert(id, task::Task {
            id,
            species,
            occurance_species,
            time_species,
            title,
            description,
            colour
        });
    }

    pub fn complete_task(&mut self, id: usize, week_start: u128, current_day: u8) -> TaskError {
        // week_start is the epoch unix of the start of monday of that week
        let task = self.library.get(&id);
        if task.is_none() {
            return TaskError::TaskNoExist;
        }
        let task = task.unwrap();

        match task.occurance_species {
            task::OccuranceSpecies::Repeating(_) => {
                let task_id = self.generate_task_id();
                let time_species = match task.time_species.clone() {
                    // START AND END ARE TREATED AS SECONDS SINCE START OF THE DAY
                    task::TimeSpecies::Start(s) => task::TimeSpecies::Start((s * 86400 * (current_day as u128)) + week_start),
                    task::TimeSpecies::Range(s, e) => task::TimeSpecies::Range((s * 86400 * (current_day as u128)) + week_start, (e * 86400 * (current_day as u128)) + week_start),
                    task::TimeSpecies::AllDay(s) => task::TimeSpecies::AllDay((s * 86400 * (current_day as u128)) + week_start),
                    task::TimeSpecies::DayRange(s, _) => task::TimeSpecies::AllDay((s * 86400 * (current_day as u128)) + week_start)
                    //                                          ^ yes this is intended
                    // repeating tasks should only be in the period of one day
                };

                self.library.insert(task_id, task::Task {
                    id: task_id,
                    species: task.species.clone(),
                    occurance_species: task::OccuranceSpecies::Once,
                    time_species,
                    title: task.title.clone(),
                    description: task.description.clone(),
                    colour: task.colour.clone()
                });
                return TaskError::Success;
            },
            _ => {}
        }

        let task = self.library.get_mut(&id).unwrap();

        match &mut task.species {
            task::Species::Task(c) => {
                *c = !*c;
                return TaskError::Success;
            }
            _ => {}
        }

        TaskError::Success
    }

    pub fn delete_task(&mut self, id: usize) -> TaskError {
        let result = self.library.get(&id);
        if result.is_none() {
            return TaskError::TaskNoExist;
        }
        self.library.remove(&id);

        TaskError::Success
    }

    pub fn update_task(&mut self, id:usize, species: task::Species, occurance_species: task::OccuranceSpecies, time_species: task::TimeSpecies, title: String, description: String, colour: u128) -> TaskError {
        let result = self.library.get_mut(&id);
        if result.is_none() {
            return TaskError::TaskNoExist;
        }
        let mut_ref = result.unwrap();
        mut_ref.species = species;
        mut_ref.occurance_species = occurance_species;
        mut_ref.time_species = time_species;
        mut_ref.title = title;
        mut_ref.description = description;
        mut_ref.colour = colour;

        TaskError::Success
    }

    fn generate_task_id(&self) -> usize {
        let r = self.library
            .clone()
            .iter()
            .map(|(x, _) | x.clone())
            .max();
        if r.is_none() {
            return 0;
        }
        r.unwrap() + 1
    }

    // from when to when
    // usually 7 days (maybe)
    pub fn fetch_library(&self, start: u128, end: u128) -> Vec<task::Task> {
        let mut r = self.library
            .iter()
            .filter(|x| x.1.in_range(start, end))
            .map(|x| x.1.clone())
            .collect::<Vec<task::Task>>();
        r.sort_by_key(|i| i.start_time());
        r
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TaskError {
    Success,

    TaskNoExist
}
impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginInformation {
    pub username: String,
    pub password: String
}
impl LoginInformation {
    // handles anything to do with password or logging in
    pub fn login(&self, account_handler: &mut Database) -> LoginResult {
        // check soterius if it exists first

        // if exists:
        //      if exists in chronos -> get user id and proceed
        //      if not exists in chronos -> generate new user id and proceed
        // if doesnt exist -> return username no exist

        match soterius::fetch(self.username.clone()) {
            Some((user_id, password)) => {
                if password != self.password {
                    return LoginResult::PasswordWrong;
                }

                let chronos_lookup = account_handler.fetch_user_id(&self.username);
                if chronos_lookup.is_none() {
                    account_handler.users.insert(user_id, User {
                        id: user_id,
                        username: self.username.clone(),
                        library: HashMap::new()
                    });
                }

                return LoginResult::Success(user_id);
            },
            None => LoginResult::UsernameNoExist
        }
    }
}
#[rocket::async_trait]
impl<'l> FromData<'l> for LoginInformation {
    type Error = LoginInfoParseError;

    async fn from_data(_req: &'l Request<'_>, mut data: Data<'l>) -> data::Outcome<'l, Self> {
        let result = data.peek(512).await.to_vec();

        if result.is_empty() {
            // return OutCome::Error
            return Outcome::Error((
                Status::Ok,
                LoginInfoParseError::Empty
            ))
        }
        
        let result = result.iter().map(|x| (x.clone()) as char).collect::<String>();
        let result: HashMap<String, String> = serde_json::from_str(result.as_str()).unwrap();

        Outcome::Success(LoginInformation {
            username: result.get("username").unwrap().clone(),
            password: result.get("password").unwrap().clone()
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum LoginInfoParseError {
    Success,

    ParsingError,

    Empty
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum LoginResult {
    Success(u128),
    UsernameNoExist,
    PasswordNoExist, // consistency issue

    PasswordWrong,

    UsernameTaken,
}
