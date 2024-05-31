use std::{collections::HashMap, fmt, fs::{self, File}, io::Read, sync::Mutex};

use rocket::State;
use serde::{Deserialize, Serialize};

use crate::user;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Database {
    pub users: HashMap<u128, user::User>
}
impl Database {
    pub fn new() -> Database {
        Database {
            users: HashMap::new()
        }
    }

    pub fn fetch_user_id(&self, username: &String) -> Option<u128> {
        for (_, u) in &self.users {
            if u.username == *username {
                return Some(u.id);
            }
        }
        None
    }

    pub fn load() -> Database {
        let mut result = Database::new();

        let mut buffer = "".to_string();
        File::open("data/users.json").unwrap().read_to_string(&mut buffer).unwrap();
        result.users = serde_json::from_str(buffer.as_str()).unwrap();

        result
    }

    pub fn save(&mut self) {
        fs::write("data/users.json", serde_json::to_string_pretty(&self.users).unwrap()).unwrap();
    }
}
impl fmt::Display for Database {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum AccountResult {
    Success(u128),
    UsernameNoExist,
    UserIDNoExist,
    PasswordWrong,

    UsernameTaken
}
impl fmt::Display for AccountResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}", self.to_string())
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

// #region api calls
#[get("/")]
pub fn debug(db: &State<Mutex<Database>>) -> String {
    let db = db.lock().unwrap();
    format!("{:?}", db)
}

#[get("/")]
pub fn load(db: &State<Mutex<Database>>) -> String {
    let mut db = db.lock().unwrap();
    *db = Database::load();
    "success".to_string()
}

#[get("/")]
pub fn save(db: &State<Mutex<Database>>) -> String {
    let mut db = db.lock().unwrap();
    db.save();
    "success".to_string()
}
// #endregion
