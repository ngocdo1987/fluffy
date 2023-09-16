use std::time::SystemTime;
use chrono::{prelude::*, NaiveDateTime};

/// get current time stamp
#[inline]
pub fn timestamp() -> u64 { 
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) =>  { n.as_secs() },
        Err(_) =>  { 0 }
    }
}

/// Require input: 2019-11-11 10:10:10
#[inline]
pub fn from_str(datetime_str: &str) -> DateTime<Local> { 
    let date_time_arr = datetime_str.split(" ").collect::<Vec<&str>>();
    let y_m_d = date_time_arr[0].split("-").collect::<Vec<&str>>();
    let h_m_s = date_time_arr[1].split(":").collect::<Vec<&str>>();
    let year = if let Ok(v) = y_m_d[0].parse::<i32>() { v } else { 1970 };
    let month = if let Ok(v) = y_m_d[1].parse::<u32>() { v } else { 1 } ;
    let day = if let Ok(v) = y_m_d[2].parse::<u32>() { v } else { 1 } ;
    let hour = if let Ok(v) = h_m_s[0].parse::<u32>() { v } else { 1 };
    let minute = if let Ok(v) = h_m_s[1].parse::<u32>() { v } else { 1 };
    let second = if let Ok(v) = h_m_s[2].parse::<u32>() { v } else { 1 };
    Local.with_ymd_and_hms(year, month, day, hour, minute, second).unwrap()
}

/// Current time string
#[inline]
pub fn to_string() -> String { 
    let local: DateTime<Local> = Local::now();
    local.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Formatting time
#[inline]
pub fn format(format_str: &str) -> String { 
    let local: DateTime<Local> = Local::now();
    local.format(format_str).to_string()
}

/// Get the current date
#[inline]
pub fn now() -> DateTime<Local> { 
    Local::now()
}

/// Get time -time
#[inline]
pub fn time() -> (u32, u32, u32) { 
    let now = now();
    (now.hour(), now.minute(), now.second())
}

/// Get the year
#[inline]
pub fn date() -> (u32, u32, u32) { 
    let now = now();
    (now.year() as u32, now.month() as u32, now.day() as u32)
}

/// Convert time stamps into date time format
#[inline]
pub fn datetime(timestamp: i64) -> String { 
    let dt = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}
