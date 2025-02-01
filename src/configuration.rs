use std::env;
use std::fs::{self};
use std::io::Write;
use std::{fs::File, io::Read, path::PathBuf};

use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, NaiveTime, TimeZone};
use chrono_tz::Tz;
use dirs::home_dir;
use serde::{Deserialize, Serialize};

use crate::view::TableView;

static ENV_NAME: &str = "CRONPLANER_CONFIG_DIR";

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct TimeSlot {
    pub name: String,
    pub date: NaiveDate,

    #[serde(default = "default_time")]
    pub time: NaiveTime,

    #[serde(default = "default_time_zone")]
    pub time_zone: Tz,
}

impl Ord for TimeSlot {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_date_time = self.to_date_time();
        let other_date_time = other.to_date_time();
        self_date_time.cmp(&other_date_time)
    }
}

impl PartialOrd for TimeSlot {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TimeSlot {
    pub fn to_date_time(&self) -> DateTime<Tz> {
        let naive_date_time = self.date.and_time(self.time);

        self.time_zone
            .from_local_datetime(&naive_date_time)
            .earliest()
            .unwrap_or_else(|| {
                self.time_zone
                    .from_local_datetime(&naive_date_time)
                    .latest()
                    .expect("Time conversion failed")
            })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TimeSlotsConfig {
    pub time_slots: Vec<TimeSlot>,
}

impl TimeSlotsConfig {
    pub fn compare_to_today(&self) -> Vec<TableView> {
        self.time_slots
            .iter()
            .map(|val| {
                let view: TableView = val.clone().into();
                view
            })
            .filter(|val| val.diff.num_seconds() >= 0)
            .collect()
    }

    fn get_appointments_path() -> PathBuf {
        let config_dir_path = Self::get_config_dir();
        config_dir_path.join("appointments.toml")
    }

    fn get_config_dir() -> PathBuf {
        env::var(ENV_NAME).map(PathBuf::from).unwrap_or_else(|_| {
            home_dir()
                .expect("Couldn't find home dir")
                .join(".config")
                .join("cronplaner")
        })
    }

    pub fn new() -> Result<Self> {
        #[cfg(feature = "example")]
        env::set_var(ENV_NAME, "./example");

        let config_path = Self::get_appointments_path();
        let mut file = File::open(&config_path)
            .unwrap_or_else(|_| panic!("Couldn't find file in: {:?}", &config_path));
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let time_slots_config: TimeSlotsConfig = toml::from_str(&contents)?;

        Ok(time_slots_config)
    }

    pub fn write_back_to_file(&self) -> Result<()> {
        let mut fd = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(Self::get_appointments_path())?;

        let mut data = self.clone();
        data.time_slots.sort();
        let to_write_data = toml::to_string(&data)?;
        fd.write_all(to_write_data.as_bytes())
            .context("Couldn't write to config file")?;

        Ok(())
    }
}

fn default_time_zone() -> Tz {
    let name = match localzone::get_local_zone() {
        Some(name) => name,
        None => "UTC".to_string(),
    };

    name.parse().unwrap()
}

fn default_time() -> NaiveTime {
    NaiveTime::from_hms_opt(0, 0, 0).expect("This should not crash")
}
