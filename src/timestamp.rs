use chrono::prelude::*;
use std::ops::{Add, Sub};
use chrono::{NaiveDate, NaiveDateTime, Utc, Local, DateTime, TimeZone, Date};

#[derive(Clone, Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct Timestamp(pub u32);

trait ToTs {
  fn to_timestamp(&self) -> Timestamp;
}

impl Timestamp {
  pub fn from_seconds(seconds: u32) -> Self {
    Timestamp(seconds)
  }
}

impl Default for Timestamp {
  fn default() -> Self {
    Timestamp(Utc::now().timestamp() as u32)
  }
}

impl Timestamp {
  pub fn to_utc_datetime(&self) -> DateTime<Utc> {
    Utc.timestamp(self.0 as i64, 0)
  }

  pub fn to_local_datetime(&self) -> DateTime<Local> {
    Local.timestamp(self.0 as i64, 0)
  }
}

impl ToTs for DateTime<Utc> {
  fn to_timestamp(&self) -> Timestamp {
    Timestamp(self.timestamp() as u32)
  }
}

impl Add for Timestamp {
  type Output = Timestamp;

  fn add(self, rhs: Timestamp) -> Timestamp {
    Timestamp(self.0 + rhs.0)
  }
}

impl Sub for Timestamp {
  type Output = Timestamp;

  fn sub(self, rhs: Timestamp) -> Timestamp {
    Timestamp(self.0 - rhs.0)
  }
}