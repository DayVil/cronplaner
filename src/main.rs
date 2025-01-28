mod configuration;

use anyhow::Result;
use configuration::TimeSlotsConfig;

fn main() -> Result<()> {
    let time_slots = TimeSlotsConfig::new()?;
    println!("Here");
    time_slots.write_back_to_file()?;

    println!("desirial: {:?}", time_slots);

    Ok(())
}
