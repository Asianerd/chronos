use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize};

use crate::task;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct User {
    pub id: u128,
    pub name: String,
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
