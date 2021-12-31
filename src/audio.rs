use std::process::Command;

pub fn get() -> (u16, bool) {
    fn work() -> Option<(u16, bool)> {
        let output = Command::new("amixer")
            .args(["-D", "pulse", "get", "Master"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
            .ok()?;

        let last_line = &output.lines().last()?;

        let data = last_line
            .split_whitespace()
            .filter(|x| x.starts_with('[') && !x.contains("dB"))
            .map(|x| x.trim_matches(&['[', ']', '%'] as &[_]))
            .collect::<Vec<&str>>();

        let volume = data.get(0)?.parse::<u16>().ok()?;

        let enabled = *data.get(1)? == "on";

        Some((volume, enabled))
    }
    work().unwrap_or((0_u16, false))
}
