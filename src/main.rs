mod configuration;

use anyhow::Result;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use configuration::TimeSlotsConfig;

fn main() -> Result<()> {
    let time_slots = TimeSlotsConfig::new()?;

    let curr_time = current_time();
    dbg!(curr_time);
    let diffs = time_slots.compare_to(&curr_time);
    let time_diffs: Vec<String> = diffs
        .iter()
        .map(|x| {
            let hours = x.num_hours();
            let form = if hours < 24 {
                let minutes = x.num_minutes() % 60;
                format!("{:02}:{:02}", hours, minutes)
            } else {
                let days = x.num_days();
                format!("{}", days)
            };
            return form;
        })
        .collect();

    dbg!(time_diffs);
    time_slots.write_back_to_file()?;
    Ok(())
}

fn current_time() -> DateTime<Tz> {
    let tz: Tz = localzone::get_local_zone()
        .expect("Can't fail")
        .parse()
        .expect("Validated inside crate");
    let current_time: DateTime<Tz> = Utc::now().with_timezone(&tz);
    current_time
}
