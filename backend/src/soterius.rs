use std::{collections::HashMap, fs};

pub fn fetch(username: String) -> Option<(u128, String)> {
    let soterius: HashMap<String, (u128, String)> = serde_json::from_str(fs::read_to_string("../../data/users.json").unwrap().as_str()).unwrap();
    soterius.get(&username).map_or_else(|| None, |e| Some((e.0.clone(), e.1.clone())))
}
