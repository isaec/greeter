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

pub fn get() -> String {
  let raw = fs::read_to_string("/proc/uptime").unwrap();
  let float_minutes = raw
    .split_whitespace()
    .next()
    .unwrap()
    .parse::<f64>()
    .unwrap()
    / 60 as f64;
  let years = Time::new("year", float_minutes / (60 * 24 * 365) as f64);
  let days = Time::new("day", float_minutes / (60 * 24) as f64);
  let hours = Time::new(
    "hour",
    (float_minutes % (float_minutes * (60 * 24) as f64)) / 60 as f64,
  );
  let minutes = Time::new(
    "minute",
    float_minutes - ((days.value * 60 * 24) + (hours.value * 60)) as f64,
  );
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
