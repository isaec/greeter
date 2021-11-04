use std::fs;

fn plural(amount: u16, label: &str) -> String {
  format!(
      "{amount} {label}{plural}",
      amount = amount,
      label = label,
      plural = if amount != 1 { "s" } else { "" }
  )
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
  let years = (float_minutes / (60 * 24 * 365) as f32) as u16;
  let days = (float_minutes / (60 * 24) as f32) as u16;
  let hours = ((float_minutes - (days * (60 * 24)) as f32) / 60 as f32) as u16;
  let minutes = (float_minutes - ((days * 60 * 24) + (hours * 60)) as f32) as u16;
  if years > 0 {
      format!(
          "{}, {}, {}, and {}",
          plural(years, "year"),
          plural(days, "day"),
          plural(hours, "hour"),
          plural(minutes, "minute")
      )
  } else if days > 0 {
      format!(
          "{}, {}, and {}",
          plural(days, "day"),
          plural(hours, "hour"),
          plural(minutes, "minute")
      )
  } else if hours > 0 {
      format!(
          "{} and {}",
          plural(hours, "hour"),
          plural(minutes, "minute")
      )
  } else {
      format!("{} minutes", plural(years, "year"))
  }
}