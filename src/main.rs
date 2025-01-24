mod configuration;

use anyhow::Result;
use chrono::NaiveDate;
use configuration::{Config, TimeSlot};
use std::env;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    let path_to_config_dir: PathBuf = env::var("CRONPLANER_CONFIG_DIR")
        .and_then(move |p| {
            let n_p: PathBuf = p.into();
            Ok(n_p)
        })
        .unwrap_or(Path::new("~").join(".config").join("cronplaner"));
    let date1 = NaiveDate::from_ymd_opt(2025, 02, 02).unwrap();

    let config = Config {
        id: 12,
        time_slots: vec![TimeSlot {
            name: "One".into(),
            date: date1,
        }],
    };

    let toml_t = toml::to_string(&config).unwrap();
    println!("Days: {:#?}", toml_t);
    println!("Config Path: {:#?}", path_to_config_dir);

    Ok(())
}
