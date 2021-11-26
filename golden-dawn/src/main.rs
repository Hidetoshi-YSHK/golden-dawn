//#![windows_subsystem = "windows"]

use std::io;
use std::fs;
use std::path::PathBuf;
use std::env::current_exe;
use chrono::prelude::*;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Config {
    tmp_dir: String,
    days_to_move: u32,
    days_to_remove: u32,
}

fn main() {
    let exe_dir = get_exe_dir().expect("Could not get exe directory.");

    create_today_dir(&exe_dir, "%Y-%m-%d")
        .expect("Could not create today's directory.");

    let dirs = get_dirs(&exe_dir).expect("Cound not get directories.");
    for dir in dirs {
        println!("{}", dir.display());
    }
}

fn get_exe_dir() -> io::Result<PathBuf> {
    let mut path_buf = current_exe()?;
    path_buf.pop();
    Ok(path_buf)
}

fn create_today_dir(parent_dir: &PathBuf, format: &str) -> io::Result<()> {
    let today = Local::today();
    let today_dir_name = today.format(format).to_string();
    let mut path_buf = parent_dir.clone();
    path_buf.push(&today_dir_name);
    if path_buf.exists() {
        Ok(())
    } else {
        fs::create_dir(path_buf)
    }
}

fn get_dirs(parent_dir: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let read_dir = fs::read_dir(parent_dir)?;
    let mut dirs = Vec::new();
    for dir_entry in read_dir {
        let dir_entry = dir_entry?;
        let metadata = dir_entry.metadata()?;
        if metadata.is_dir() {
            dirs.push(dir_entry.path());
        }
    }
    Ok(dirs)
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