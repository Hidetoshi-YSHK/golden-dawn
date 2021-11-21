//#![windows_subsystem = "windows"]

use std::fs;

use chrono::prelude::*;

fn main() {
    let today = Local::today();
    let today_dir_name = today.format("%Y-%m-%d").to_string();
    fs::create_dir(today_dir_name).ok();
}
