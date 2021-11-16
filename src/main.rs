extern crate termion;

use std::fs;
use std::cmp::{min, max};
use termion::color;

mod bar;
use bar::Color;
use bar::ColorRange;
mod uptime;
mod audio;

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

const BAR_WIDTH: u8 = 25;

const BAT_CAPACITY: &str = "/sys/class/power_supply/BAT0/capacity"; // ideally would use BAT*
const BAT_STATUS: &str = "/sys/class/power_supply/BAT0/status";
const ACTUAL_BACKLIGHT: &str = "/sys/class/backlight/amdgpu_bl0/actual_brightness";
const MAX_BACKLIGHT: &str = "/sys/class/backlight/amdgpu_bl0/max_brightness";

fn main() {
    let red_to_green = ColorRange::new3(Color::rgb(230, 0, 115), Color::rgb(230, 230, 60), Color::rgb(50, 200, 50));
    let blue_to_mag = ColorRange::new2(Color::rgb(0, 175, 230), Color::rgb(230, 0, 115));
    let yellow_to_red = ColorRange::new2(Color::rgb(230, 230, 100), Color::rgb(230, 100, 150));
    let default = color::Fg(color::Rgb(150, 152, 150));

    let sys_batt_percent = read_val(BAT_CAPACITY);
    // &fn[..] converts String to &str for easy matching
    let batt_status = match &read_val_str(BAT_STATUS)[..] {
        "Charging" => format!(
            "{c}charging battery{r} at {c}{}%{r}",
            sys_batt_percent,
            c = red_to_green.get_color(max(70, sys_batt_percent) as f32 / 100_f32),
            r = default,
        ),
        "Full" => format!("{}full battery{}", red_to_green.get_color(1.0), default),
        _ => format!(
            "{c}draining battery{r} at {c}{}%{r}",
            sys_batt_percent,
            c = red_to_green.get_color(min(30, sys_batt_percent) as f32 / 100_f32),
            r = default,
        ),
    };
    let display_percent =
        ((read_val(ACTUAL_BACKLIGHT) as f32 / read_val(MAX_BACKLIGHT) as f32) * 100.0).round() as u16;

    let (audio_level, audio_enabled) = audio::get();

    // for n in 0..=100 {
    //     if n % 2 == 0 {
    //         println!("{} {}", bar::make(30, n, &red_to_green, "<", "/", "-", ">", &default), n);
    //     }
    // }

    println!(
        "{default}running at {sys_temp}c
on {kernel_vers}
for {uptime}
at {display_percent}% brightness
{display_bar}
{audio_bar}
sound {audio_state} at {audio_level}% volume
with a {batt_status}
{batt_bar}
{reset}",
        sys_temp = read_val("/sys/class/thermal/thermal_zone0/temp") / 1000, // celsius
        uptime = uptime::get(),
        display_percent = display_percent,
        display_bar = bar::make(BAR_WIDTH, display_percent, &yellow_to_red, "<", "/", "-", ">", &default),
        audio_bar = bar::make(BAR_WIDTH, audio_level, &blue_to_mag, "<", "\\", "-", ">", &default),
        audio_state = if audio_enabled { "on" } else { "muted" },
        audio_level = audio_level,
        batt_status = batt_status,
        batt_bar = bar::make(30, sys_batt_percent, &red_to_green, "|", "=", "-", "|", &default),
        kernel_vers = read_val_str("/proc/sys/kernel/osrelease"), // equivalent to uname -r
        default = default,
        reset = color::Fg(color::Reset),
    );
}
