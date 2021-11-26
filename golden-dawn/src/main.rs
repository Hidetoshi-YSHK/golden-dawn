//#![windows_subsystem = "windows"]

use std::io;
use std::fs;
use std::path::Path;
use std::env::current_exe;
use chrono::LocalResult;
use chrono::prelude::*;
use serde_derive::Deserialize;

const CONFIG_FILE: &str = "config.toml";
const OLD_DIR: &str = "old";

#[derive(Deserialize)]
struct Config {
    parent_dir: String,
    date_format: String,
    days_to_move: u32,
    days_to_remove: u32,
}

fn main() {
    let config_str = read_config_file().expect("Could not read config file.");
    let config = toml::from_str::<Config>(&config_str)
        .expect("Could not parse config file.");

    let parent_dir = Path::new(&config.parent_dir);

    create_today_dir(parent_dir, &config.date_format)
        .expect("Could not create today's directory.");

    let dir_names = get_dir_names(parent_dir).expect("Cound not get directories.");
    for dir_name in dir_names {
        if !matches_date_format(&dir_name, &config.date_format) {
            continue;
        }
        let days_elapsed = calc_days_elapsed(&dir_name, &config.date_format)
            .expect("Failed to calc days elapsed.");
        if days_elapsed >= config.days_to_move.into() {

        }
    }
}

fn read_config_file() -> io::Result<String> {
    let mut exe_dir = current_exe()?;
    exe_dir.pop();

    let config_file = Path::join(&exe_dir, CONFIG_FILE);
    Ok(fs::read_to_string(config_file)?)
}

fn create_today_dir(parent_dir_path: &Path, date_format: &String) -> io::Result<()> {
    let today = Local::today();
    let today_dir_name = today.format(date_format).to_string();
    let mut path_buf = parent_dir_path.to_path_buf();
    path_buf.push(&today_dir_name);
    if path_buf.exists() {
        Ok(())
    } else {
        fs::create_dir(path_buf)
    }
}

fn get_dir_names(parent_dir: &Path) -> io::Result<Vec<String>> {
    let read_dir = fs::read_dir(parent_dir)?;
    let mut dir_names = Vec::new();
    for dir_entry in read_dir {
        let dir_entry = dir_entry?;
        let metadata = dir_entry.metadata()?;
        if metadata.is_dir() {
            if let Ok(dir_name) = dir_entry.file_name().into_string() {
                dir_names.push(dir_name);
            }
        }
    }
    Ok(dir_names)
}

fn matches_date_format(dir_name: &String, date_format: &String) -> bool {
    match NaiveDate::parse_from_str(dir_name, date_format) {
        Ok(_) => true,
        Err(_) => false
    }
}

fn calc_days_elapsed(dir_name: &String, date_format: &String) ->
    Result<i64, String> {
    let date = NaiveDate::parse_from_str(dir_name, date_format)
        .map_err(|_| "Failed to parse dir name.".to_string())?;
    if let LocalResult::Single(local_date) = Local.from_local_date(&date) {
        Ok((Local::today() - local_date).num_days())
    } else {
        Err("Failed to convert date.".to_string())
    }
}

fn move_to_old_dir(parent_dir: &Path, dir_name: &String) {
    let target_dir = parent_dir.join(dir_name);
    let old_dir = parent_dir.join(OLD_DIR);
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