mod configuration;

use anyhow::Result;
use configuration::TimeSlotsConfig;

fn main() -> Result<()> {
    let time_slots = TimeSlotsConfig::new()?;

    println!("desirial: {:?}", time_slots);

    Ok(())
}
