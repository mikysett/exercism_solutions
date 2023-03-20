use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const MIN_PER_DAY: i32 = 1440;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let time_in_min = ((hours * 60 + minutes)).rem_euclid(MIN_PER_DAY);
        Self {
            hours: time_in_min / 60,
            minutes: time_in_min % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
