use chrono::{Datelike, Duration, Local};
use log::info;

// pub const GROUP_ID: i64 = -1063900471;
pub const GROUP_ID: i64 = -281597102;

pub fn get_todays_date() -> (i32, u32, u32) {
    let current_date = chrono::Utc::now().date_naive();
    (
        current_date.year(),
        current_date.month(),
        current_date.day(),
    )
}

pub fn calculate_next_midnight() -> u64 {
    let now = Local::now();
    let tomorrow_midnight = (now + Duration::days(1)).date_naive().and_hms_opt(0, 0, 0);
    let duration = tomorrow_midnight
        .unwrap()
        .signed_duration_since(now.naive_local())
        .to_std()
        .unwrap();
    info!(
        "Duration between {:?} and {:?}: {:?}",
        now, tomorrow_midnight, duration
    );
    duration.as_secs()
}

pub fn is_thursday() -> bool {
    let current_time = chrono::offset::Local::now();
    current_time.date_naive().weekday() == chrono::Weekday::Thu
}
