use chrono::{DateTime, TimeDelta, Utc};
use chrono_tz::Tz;

use crate::configuration::TimeSlot;

#[derive(Debug, Clone)]
pub struct TableView {
    pub name: String,
    pub diff: TimeDelta,
    pub due: DateTime<Tz>,
}

impl TableView {
    pub fn less_one_day(&self) -> bool {
        self.diff.num_hours() < 24
    }

    pub fn to_days(&self) -> String {
        format!("{}", self.diff.num_days())
    }

    pub fn to_hours(&self) -> String {
        let hours = self.diff.num_hours() % 24;
        let minutes = self.diff.num_minutes() % 60;
        format!("{:02}:{:02}", hours, minutes)
    }
}

impl From<TimeSlot> for TableView {
    fn from(val: TimeSlot) -> Self {
        let current_time = current_time();
        let diff = val.to_date_time().with_timezone(&Utc) - current_time.with_timezone(&Utc);
        let due = val.to_date_time();
        let name = val.name;
        TableView { name, diff, due }
    }
}

fn current_time() -> DateTime<Tz> {
    let tz: Tz = localzone::get_local_zone()
        .expect("Can't fail")
        .parse()
        .expect("Validated inside crate");
    let current_time: DateTime<Tz> = Utc::now().with_timezone(&tz);
    current_time
}
