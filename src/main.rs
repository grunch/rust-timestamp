// use chrono::prelude::*;
mod timestamp;
use timestamp::Timestamp;
use chrono::{NaiveDate, NaiveDateTime, Utc, Local, DateTime, Datelike};
use chrono::TimeZone;

fn main() {
    let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    // let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));
    let ts = Timestamp::default();
    let two_min = Timestamp::from_seconds(120);
    // let x = "2014-11-28T12:00:09Z";
    // let y = DateTime::from_str(x);
    // let x = ts.to_utc();
    // let y = ts.to_local();
    // let ndt = NaiveDateTime::from_timestamp(1633355675, 0);
    let new_ts = ts + two_min;
    println!("{:?}", new_ts);
    let new_ts_local = new_ts.to_local_datetime();
    println!("{:?}", new_ts_local);
    // let yy = xx.timestamp();
    // let current_time = chrono::offset::Local::now();
    // println!("{:?}", yy);
    // println!("{}", current_time.date());
    // println!("{:?}", current_time);
    // println!("{:?}", Utc::today());
    // println!("{:?}", Local::today());
    // println!("{:?}", Utc.ymd(2015, 5, 15).to_string());
}
