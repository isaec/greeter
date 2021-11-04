use std::fmt;
use std::fs;

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

pub fn get() -> String {
  let float_minutes = fs::read_to_string("/proc/uptime")
    .unwrap()
    .split_whitespace()
    .next()
    .unwrap()
    .parse::<f64>()
    .unwrap()
    / 60 as f64;
  let years = Time::new("year", float_minutes / YEAR);
  let days = Time::new("day", float_minutes % YEAR / DAY);
  let hours = Time::new("hour", float_minutes % DAY / HOUR);
  let minutes = Time::new("minute", float_minutes % HOUR);

  if years.value > 0 {
    format!("{}, {}, {}, and {}", years, days, hours, minutes)
  } else if days.value > 0 {
    format!("{}, {}, and {}", days, hours, minutes)
  } else if hours.value > 0 {
    format!("{} and {}", hours, minutes)
  } else {
    format!("{}", minutes)
  }
}
