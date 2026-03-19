use chrono::{DateTime, SecondsFormat, Utc};
use std::fmt;
use crate::query_list::Comparison;

#[derive(Clone)]
pub struct TimeCreated {
    time: DateTime<Utc>,
    comparison: Comparison,
}

impl TimeCreated {
    pub fn new(time: DateTime<Utc>, comparison: Comparison) -> TimeCreated {
        TimeCreated { time, comparison }
    }
}

impl fmt::Display for TimeCreated {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let iso_time = self.time.to_rfc3339_opts(SecondsFormat::Millis, true);
        write!(
            f,
            "TimeCreated[@SystemTime {} '{}']",
            self.comparison, iso_time
        )
    }
}
