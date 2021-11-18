use std::fmt;
use std::fs;

use chrono::prelude::Local;

struct Time<'a> {
    label: &'a str,
    value: u64,
}

impl Time<'_> {
    fn new<'a>(label: &'a str, value: f64) -> Time<'_> {
        Time {
            label,
            value: value as u64,
        }
    }
}

impl fmt::Display for Time<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{value} {label}{plural}",
            value = self.value,
            label = self.label,
            plural = if self.value == 1 { "" } else { "s" }
        )
    }
}

const HOUR: f64 = 60 as f64;
const DAY: f64 = HOUR * 24 as f64;
const YEAR: f64 = DAY * 365 as f64;

pub fn get_uptime() -> String {
    let float_minutes = fs::read_to_string("/proc/uptime")
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<f64>()
        .unwrap()
        / 60 as f64;

    let mut time = vec![
        Time::new("year", float_minutes / YEAR),
        Time::new("day", float_minutes % YEAR / DAY),
        Time::new("hour", float_minutes % DAY / HOUR),
        Time::new("minute", float_minutes % HOUR),
    ];
    time.retain(|unit| unit.value != 0);
    let len = time.len() - 1;
    let mut result = "".to_string();
    for (i, unit) in time.iter().enumerate() {
        result = format!(
            "{}{}{}",
            result,
            unit,
            if i == len {
                ""
            } else if i == len - 1 {
                " and "
            } else {
                ", "
            }
        );
    }

    result
}

pub fn get_date_time() -> String {
    Local::now()
        .format("at %-I:%M %P on %A, %B %d, %Y")
        .to_string()
        .to_lowercase()
}
