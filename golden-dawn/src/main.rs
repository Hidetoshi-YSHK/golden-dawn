//#![windows_subsystem = "windows"]

use std::fs;
use chrono::prelude::*;

fn main() {
    create_today_dir();
    move_old_dir();
}

fn create_today_dir() {
    let today = Local::today();
    let today_dir_name = today.format("%Y-%m-%d").to_string();
    fs::create_dir(today_dir_name).ok();
}

fn move_old_dir() {
    let read_dir = fs::read_dir(".").unwrap();
    for dir_entry in read_dir {
        let dir_entry = dir_entry.unwrap();
        let metadata = dir_entry.metadata().unwrap();
        if metadata.is_dir() {
            let dir_name = dir_entry.file_name();
            let dir_name = dir_name.into_string().unwrap();
            println!("{}", dir_name);
            let result = NaiveDate::parse_from_str(
                dir_name.as_str(), "%Y-%m-%d");
            match result {
                Ok(date) => {
                    let date = Local.from_local_date(&date).unwrap();
                    let diff = Local::today() - date;
                    let days = diff.num_days();
                    println!("{}", days);
                },
                _ => ()
            }
        }
    }
}