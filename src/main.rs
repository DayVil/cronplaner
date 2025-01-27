mod configuration;

use anyhow::Result;
use chrono::NaiveDate;
use configuration::{TimeSlot, TimeSlotsConfig};

fn main() -> Result<()> {
    let time_slots = TimeSlotsConfig::new()?;

    let d = TimeSlotsConfig {
        time_slots: vec![TimeSlot {
            name: "Hello".to_string(),
            date: NaiveDate::from_ymd_opt(2002, 02, 23).unwrap(),
        }],
    };
    println!("serial: {:?}", toml::to_string(&d));
    println!("desirial: {:?}", time_slots);

    Ok(())
}
