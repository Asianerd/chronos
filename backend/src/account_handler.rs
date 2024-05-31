use std::{collections::HashMap, fmt, fs::{self, File}, io::Read, sync::Mutex};

use rocket::State;
use serde::{Deserialize, Serialize};

use crate::{login_info::LoginInformation, user};

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

    fn passwords() -> HashMap<u128, String> {
        serde_json::from_str(fs::read_to_string("data/passwords.json").unwrap().as_str()).unwrap()
    }

    pub fn username_exists(&self, username: &String) -> bool {
        for (_, u) in &self.users {
            if u.name == *username {
                return true;
            }
        }
        false
    }

    pub fn fetch_user_id(&self, username: &String) -> Option<u128> {
        for (_, u) in &self.users {
            if u.name == *username {
                return Some(u.id);
            }
        }
        None
    }

    pub fn login(&self, login: &LoginInformation) -> AccountResult {
        let user_id = self.fetch_user_id(&login.username);
        if user_id.is_none() {
            return AccountResult::UsernameNoExist;
        }

        let passwords = Database::passwords();
        let fetch_result = passwords.get(&user_id.unwrap());
        if fetch_result.is_none() {
            return AccountResult::UserIDNoExist;
        }
        if fetch_result.unwrap() == &login.password {
            return AccountResult::Success(user_id.unwrap());
        }

        AccountResult::PasswordWrong
    }

    pub fn signup(&mut self, login: &LoginInformation) -> AccountResult {
        if self.username_exists(&login.username) {
            return AccountResult::UsernameTaken;
        }

        let user_id = self.generate_user_id();

        self.users.insert(user_id,user::User {
            id: user_id,
            name: login.username.clone(),
            library: HashMap::new(),
            // repeating_tasks: vec![]
            // library: HashMap::new()
        });

        Database::add_password(user_id, &login.password);

        self.save();

        AccountResult::Success(user_id)
    }

    fn generate_user_id(&self) -> u128 {
        self.users.len() as u128
    }

    fn add_password(id: u128, password: &String) {
        let mut passwords = Database::passwords();
        passwords.insert(id, password.clone());
        fs::write("data/passwords.json", serde_json::to_string_pretty(&passwords).unwrap()).unwrap();
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
