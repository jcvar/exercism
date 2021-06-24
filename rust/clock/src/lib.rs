use std::fmt;
#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    h: i32,
    m: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.h, self.m)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // unimplemented!(
        //     "Construct a new Clock from {} hours and {} minutes",
        //     hours,
        //     minutes
        // );
        let h = (hours + minutes.div_euclid(60)).rem_euclid(24);
        let m = minutes.rem_euclid(60);
        Clock { h, m }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // unimplemented!("Add {} minutes to existing Clock time", minutes);
        Clock::new(self.h, self.m + minutes)
    }
}
