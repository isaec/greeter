use std::process::{Command, Stdio};

pub fn get() -> (u8, bool) {
  let output = Command::new("amixer")
    .args(&["-D", "pulse", "get", "Master"])
    .output()
    .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
    .unwrap();

  let last_line = &output.lines().last().unwrap();

  let data = last_line
    .split_whitespace()
    .filter(|x| x.starts_with('[') && !x.contains("dB"))
    .map(|x| x.trim_matches(&['[', ']', '%'] as &[_]))
    .collect::<Vec<&str>>();

  let volume = data.get(0).unwrap().parse::<u8>().unwrap();

  let enabled = *data.get(1).unwrap() != "on";

  (volume, enabled)
}
