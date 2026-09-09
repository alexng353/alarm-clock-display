use chrono::{DateTime, Timelike, Utc};
use rp2040_hal::timer::Instant;

#[derive(Clone, Copy)]
pub struct Clock {
    reference_time: DateTime<Utc>,
    reference_instant: Instant,
}

pub fn to_digits_pacific(time: DateTime<Utc>) -> [u8; 4] {
    let offset = chrono::FixedOffset::west_opt(7 * 3600).unwrap();
    let time = time.with_timezone(&offset);

    let mut digits: [u8; 4] = [0, 0, 0, 0];

    let hour = time.hour();
    let minute = time.minute();

    digits[0] = (hour / 10) as u8;
    digits[1] = (hour % 10) as u8;
    digits[2] = (minute / 10) as u8;
    digits[3] = (minute % 10) as u8;

    digits
}

impl Clock {
    pub fn new(time: DateTime<Utc>, instant: Instant) -> Self {
        Self {
            reference_time: time,
            reference_instant: instant,
        }
    }

    pub fn get_time(&self, now_instant: Instant) -> DateTime<Utc> {
        let offset = chrono::TimeDelta::microseconds(
            (now_instant - self.reference_instant).to_micros() as i64,
        );

        self.reference_time + offset
    }

    pub fn update_reference(&mut self, time: DateTime<Utc>, instant: Instant) {
        self.reference_time = time;
        self.reference_instant = instant;
    }
}
