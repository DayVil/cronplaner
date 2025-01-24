mod configuration;
use chrono::NaiveDate;
use configuration::{Config, TimeSlot};

fn main() {
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
}
