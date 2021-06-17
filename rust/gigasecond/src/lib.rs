use chrono::{DateTime, TimeZone, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    // unimplemented!("What time is a gigasecond later than {}", start);
    Utc.timestamp(start.timestamp() + 1_000_000_000, 0)
}
