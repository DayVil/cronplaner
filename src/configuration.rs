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

        let path_to_config_dir: PathBuf = Self::get_config_dir()?;
        let config_path = path_to_config_dir.join("appointments.toml");
        let mut file = File::open(&config_path).context("Couldn't find appointments.toml")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let time_slots_config: TimeSlotsConfig = toml::from_str(&contents)?;

        Ok(time_slots_config)
    }

    fn get_config_dir() -> Result<PathBuf> {
        return Ok(env::var(ENV_NAME)
            .map(PathBuf::from)
            .unwrap_or_else(|_| Path::new("~").join(".config").join("cronplaner")));
    }
}
