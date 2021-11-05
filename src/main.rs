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

const BAT_CAPACITY: &str = "/sys/class/power_supply/BAT0/capacity"; // ideally would use BAT*
const BAT_STATUS: &str = "/sys/class/power_supply/BAT0/status";
const ACTUAL_BACKLIGHT: &str = "/sys/class/backlight/amdgpu_bl0/actual_brightness";
const MAX_BACKLIGHT: &str = "/sys/class/backlight/amdgpu_bl0/max_brightness";


fn main() {
    let sys_batt_percent = read_val(BAT_CAPACITY);
    // &fn[..] converts String to &str for easy matching
    let batt_status = match &read_val_str(BAT_STATUS)[..] {
        "Charging" => format!("charging battery at {}%", sys_batt_percent),
        "Full" => format!("full battery"),
        _ => format!("draining battery at {}%", sys_batt_percent), // draining
    };
    let display_percent = ((read_val(ACTUAL_BACKLIGHT) as f32 / read_val(MAX_BACKLIGHT) as f32) * 100.0) as u16;

    let red_to_green = ColorRange::new(Color::rgb(230, 0, 115), Color::rgb(0, 175, 100));
    let blue_to_mag = ColorRange::new(Color::rgb(0, 175, 230), Color::rgb(230, 0, 115));

    // for n in 0..=100 {
    //     if n % 2 == 0 {
    //         println!("{}", bar::make(30, n, &blue_to_mag, "<", "/", "-", ">"));
    //     }
    // }

    println!(
        "running at {red}{sys_temp}c{reset}
on {kernel_vers}
for {uptime}
at {display_percent}% brightness
{display_bar}
with a {batt_status}
{batt_bar}
",
        sys_temp = read_val("/sys/class/thermal/thermal_zone0/temp") / 1000, // celsius
        uptime = uptime::get(),
        display_percent = display_percent,
        display_bar = bar::make(20, display_percent, &blue_to_mag, "<", "/", "-", ">"),
        batt_status = batt_status,
        batt_bar = bar::make(30, sys_batt_percent, &red_to_green, "|", "=", "-", "|"),
        kernel_vers = read_val_str("/proc/sys/kernel/osrelease"), // equivalent to uname -r
        red = color::Fg(color::Red),
        reset = color::Fg(color::Reset),
    );
}
