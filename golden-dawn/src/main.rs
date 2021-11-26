//#![windows_subsystem = "windows"]

use std::io;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::env::current_exe;
use chrono::prelude::*;
use serde::__private::ser::constrain;
use serde_derive::Deserialize;

const CONFIG_FILE: &str = "config.toml";

#[derive(Deserialize)]
struct Config {
    tmp_dir: String,
    date_format: String,
    days_to_move: u32,
    days_to_remove: u32,
}

fn main() {
    let config_str = read_config_file().expect("Could not read config file.");
    let config = toml::from_str::<Config>(&config_str)
        .expect("Could not parse config file.");

    let tmp_dir = Path::new(&config.tmp_dir);

    create_today_dir(tmp_dir, "%Y-%m-%d")
        .expect("Could not create today's directory.");

    let dirs = get_dirs(tmp_dir).expect("Cound not get directories.");
    for dir in dirs {
        println!("{}", dir.display());
    }
}

fn read_config_file() -> io::Result<String> {
    let mut exe_dir = current_exe()?;
    exe_dir.pop();

    let config_file = Path::join(&exe_dir, CONFIG_FILE);
    Ok(fs::read_to_string(config_file)?)
}

fn create_today_dir(parent_dir_path: &Path, format: &str) -> io::Result<()> {
    let today = Local::today();
    let today_dir_name = today.format(format).to_string();
    let mut path_buf = parent_dir_path.to_path_buf();
    path_buf.push(&today_dir_name);
    if path_buf.exists() {
        Ok(())
    } else {
        fs::create_dir(path_buf)
    }
}

fn get_dirs(parent_dir: &Path) -> io::Result<Vec<PathBuf>> {
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