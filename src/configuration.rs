use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub id: i32,
    pub time_slots: Vec<TimeSlot>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TimeSlot {
    pub name: String,
    pub date: NaiveDate,
}
