use chrono::{Month, Weekday};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Months(pub Month);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Weekdays(pub Weekday);

impl Iterator for Months {
    type Item = Month;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.0.succ();
        self.0 = next;
        Some(next)
    }
}

impl Iterator for Weekdays {
    type Item = Weekday;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.0.succ();
        self.0 = next;
        Some(next)
    }
}
