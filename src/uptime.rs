use std::fmt;
use std::fs;

struct Time<'a> {
  label: &'a str,
  value: u16,
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
    .parse::<f32>()
    .unwrap()
    / 60 as f32;
  let years = Time {
    label: "year",
    value: (float_minutes / (60 * 24 * 365) as f32) as u16,
  };
  let days = Time {
    label: "day",
    value: (float_minutes / (60 * 24) as f32) as u16,
  };
  let hours = Time {
    label: "hour",
    value: ((float_minutes - (days.value * (60 * 24)) as f32) / 60 as f32) as u16,
  };
  let minutes = Time {
    label: "minute",
    value: (float_minutes - ((days.value * 60 * 24) + (hours.value * 60)) as f32) as u16,
  };
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
