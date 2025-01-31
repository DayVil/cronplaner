mod configuration;

use anyhow::Result;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use configuration::TimeSlotsConfig;

fn main() -> Result<()> {
    let time_slots = TimeSlotsConfig::new()?;

    let curr_time = current_time();
    let diffs = time_slots.compare_to(&curr_time);
    dbg!(diffs);

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
