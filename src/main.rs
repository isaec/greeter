extern crate termion;

use std::fs;
use termion::color;

mod bar;
use bar::Color;
use bar::ColorRange;
mod uptime;

fn read_val(path: &str) -> u16 {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .parse::<u16>()
        .unwrap()
}

fn read_val_str(path: &str) -> String {
    String::from(fs::read_to_string(path).unwrap().trim())
}

fn main() {
    let sys_batt_percent = read_val("/sys/class/power_supply/BAT0/capacity"); // ideally would use BAT*
    let uptime = uptime::get();

    let red_to_green = ColorRange::new(
        Color::rgb(255, 0, 50), Color::rgb(0, 255, 50)
    );

    println!(
        "running at {red}{sys_temp}c{reset}
on {kernel_vers}
for {uptime}
percent: {sys_batt_percent}%
{batt_bar}
",
        sys_temp = read_val("/sys/class/thermal/thermal_zone0/temp") / 1000, // celsius
        uptime = uptime,
        sys_batt_percent = sys_batt_percent,
        batt_bar = bar::make(30, sys_batt_percent, red_to_green, "<", "/", "-", ">"),
        kernel_vers = read_val_str("/proc/sys/kernel/osrelease"), // equivalent to uname -r
        red = color::Fg(color::Red),
        reset = color::Fg(color::Reset),
    );
}
