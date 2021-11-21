#![windows_subsystem = "windows"]

use std::fs;

fn main() {
    fs::create_dir("20211121").ok();
}
