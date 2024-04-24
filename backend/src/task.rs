use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

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
