use std::fs;
use std::io::Write;
use std::{env, path::Path};
use std::{fs::File, io::Read, path::PathBuf};

use anyhow::{Context, Result};
use chrono::{NaiveDate, NaiveTime};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

static ENV_NAME: &str = "CRONPLANER_CONFIG_DIR";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TimeSlot {
    pub name: String,
    pub date: NaiveDate,

    #[serde(default = "default_time")]
    pub time: NaiveTime,

    #[serde(default = "default_time_zone")]
    pub time_zone: Tz,
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TimeSlotsConfig {
    pub time_slots: Vec<TimeSlot>,
}

impl TimeSlotsConfig {
    pub fn new() -> Result<Self> {
        #[cfg(feature = "example")]
        env::set_var(ENV_NAME, "./example");

        let config_path = Self::get_appointments_path()?;
        let mut file = File::open(&config_path).context(format!("{:?}", config_path))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let time_slots_config: TimeSlotsConfig = toml::from_str(&contents)?;

        Ok(time_slots_config)
    }

    fn get_appointments_path() -> Result<PathBuf> {
        let config_dir_path = Self::get_config_dir()?;
        Ok(config_dir_path.join("appointments.toml"))
    }

    fn get_config_dir() -> Result<PathBuf> {
        return Ok(env::var(ENV_NAME)
            .map(PathBuf::from)
            .unwrap_or_else(|_| Path::new("~").join(".config").join("cronplaner")));
    }

    pub fn write_back_to_file(&self) -> Result<()> {
        let mut fd = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(Self::get_appointments_path()?)?;

        let mut data = self.clone();
        data.time_slots.sort_by(|a, b| {
            let a_date_time = a.date.and_time(a.time);
            let b_date_time = b.date.and_time(b.time);
            a_date_time.cmp(&b_date_time)
        });
        println!("{:?}", data);
        let to_write_data = toml::to_string(&data)?;
        fd.write_all(to_write_data.as_bytes())?;

        Ok(())
    }
}
