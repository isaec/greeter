extern crate termion;

use std::fs;
use termion::color;

fn read_sys(path: &str) -> u16 {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .parse::<u16>()
        .unwrap()
}

fn main() {
    let sys_temp = read_sys("/sys/class/thermal/thermal_zone0/temp") / 1000; // celsius
    let sys_batt_percent = read_sys("/sys/class/power_supply/BAT0/capacity"); // ideally would use BAT*
    println!(
        "temp: {red}{}c{reset}\npercent: {}%",
        sys_temp,
        sys_batt_percent,
        red = color::Fg(color::Red),
        reset = color::Fg(color::Reset),
    );
}
